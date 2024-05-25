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
    pub(crate) range: tree_sitter::Range,
    pub(crate) format: SourceValueFormat,
}

#[derive(Clone, Debug)]
pub enum SourceValueFormat {
    String,
    Array,
}
