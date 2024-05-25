use anyhow::{bail, Result};
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

    pub fn clone_with_adjusments(
        &self,
        adjustments: &Vec<(std::ops::Range<usize>, usize)>,
    ) -> Result<EmbeddedSourceMap> {
        let mut new_map = self.clone();
        let mut section_iterator = new_map.sections.iter_mut();
        let mut current = match section_iterator.next() {
            Some(section) => section,
            None => return Ok(new_map),
        };

        let mut accumulated_offset: i32 = 0;

        for (source_range, replacement_length) in adjustments.iter().rev() {
            // Make sure we are on the right section
            while source_range.start > current.inner_range_end {
                current = match section_iterator.next() {
                    Some(section) => {
                        // Apply our accumulaed offset to the section
                        section.inner_range_end =
                            (section.inner_range_end as i32 + accumulated_offset) as usize;
                        section
                    }
                    None => return Ok(new_map),
                };
            }

            let length_diff =
                *replacement_length as i32 - (source_range.end - source_range.start) as i32;
            // Accumulate the overall offset, which we will use for future sections
            accumulated_offset += length_diff;
            // Apply the offset to the current section
            current.inner_range_end = (current.inner_range_end as i32 + length_diff) as usize;
        }
        Ok(new_map)
    }

    pub fn fill_with_inner(&self, new_inner_source: &str) -> Result<String> {
        let mut outer_source = self.outer_source.clone();

        let mut current_inner_offset = 0;
        let mut current_outer_offset = 0;

        for section in &self.sections {
            // TODO: actually get the *updated* range
            let replacement_code = new_inner_source
                .get(current_inner_offset..(section.inner_range_end - section.inner_end_trim))
                .ok_or(anyhow::anyhow!("Section range is out of bounds"))?;

            let json = section.as_json(replacement_code);

            let outer_range = (section.outer_range.start as i32 + current_outer_offset) as usize
                ..(section.outer_range.end as i32 + current_outer_offset) as usize;

            let length_diff = json.len() as i32 - (outer_range.end - outer_range.start) as i32;
            current_outer_offset += length_diff;
            current_inner_offset = section.inner_range_end;

            outer_source.replace_range(outer_range, &json);
        }

        Ok(outer_source)
    }
}

#[derive(Debug, Clone)]
pub struct SourceMapSection {
    /// The range of the code within the outer document
    pub(crate) outer_range: ByteRange,
    /// The end of the range from the inner document
    pub(crate) inner_range_end: usize,
    pub(crate) format: SourceValueFormat,
    /// Content we should trim from the inner document before inserting back in (ex. "\n" at the end)
    pub(crate) inner_end_trim: usize,
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
