use anyhow::Result;
use grit_util::ByteRange;
use serde_json::json;

/// A source map is used when the code we are parsing is embedded inside a larger file.
/// For example, we want to focus on the Python code inside a Jupyter notebook.
#[derive(Debug, Clone)]
pub struct EmbeddedSourceMap {
    sections: Vec<SourceMapSection>,
    /// This is a bit suboptimal, but we assume nobody has tons of embedded files
    pub(crate) outer_source: String,
}

impl EmbeddedSourceMap {
    pub fn new(outer_source: &str) -> Self {
        Self {
            sections: vec![],
            outer_source: outer_source.to_string(),
        }
    }

    pub fn add_section(&mut self, section: SourceMapSection) {
        self.sections.push(section);
    }

    pub fn fill_with_inner(&self, new_inner_source: &str) -> Result<String> {
        let mut outer_source = self.outer_source.clone();

        for section in &self.sections {
            // TODO: actually get the *updated* range
            let replacement_code = new_inner_source
                .get(section.inner_range.start..section.inner_range.end)
                .ok_or(anyhow::anyhow!("Section range is out of bounds"))?;

            let json = section.as_json(replacement_code);

            outer_source.replace_range(section.outer_range.start..section.outer_range.end, &json);
        }

        Ok(outer_source)
    }
}

#[derive(Debug, Clone)]
pub struct SourceMapSection {
    /// The range of the code within the outer document
    pub(crate) outer_range: ByteRange,
    /// The range of the code inside the inner document
    pub(crate) inner_range: ByteRange,
    pub(crate) format: SourceValueFormat,
}

impl SourceMapSection {
    pub fn as_json(&self, code: &str) -> String {
        let structure = match self.format {
            SourceValueFormat::String => serde_json::Value::String(code.to_string()),
            SourceValueFormat::Array => {
                json!(vec![code])
            }
        };
        structure.to_string()
    }
}

#[derive(Clone, Debug)]
pub enum SourceValueFormat {
    String,
    Array,
}
