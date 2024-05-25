/// A source map is used when the code we are parsing is embedded inside a larger file.
/// For example, we want to focus on the Python code inside a Jupyter notebook.
pub struct EmbeddedSourceMap {}

pub struct SourceMapSection {
    pub(crate) range: tree_sitter::Range,
    pub(crate) format: SourceValueFormat,
}

#[derive(Clone, Debug)]
pub enum SourceValueFormat {
    String,
    Array,
}
