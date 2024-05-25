use grit_util::ByteRange;

/// A source map is used when the code we are parsing is embedded inside a larger file.
/// For example, we want to focus on the Python code inside a Jupyter notebook.
#[derive(Debug, Clone)]
pub struct EmbeddedSourceMap {
    sections: Vec<SourceMapSection>,
}

impl EmbeddedSourceMap {
    pub fn new() -> Self {
        Self { sections: vec![] }
    }

    pub fn add_section(&mut self, section: SourceMapSection) {
        self.sections.push(section);
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

#[derive(Clone, Debug)]
pub enum SourceValueFormat {
    String,
    Array,
}
