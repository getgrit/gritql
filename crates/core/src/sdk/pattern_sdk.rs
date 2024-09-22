/// UncompiledPattern is used to build up complex patterns *before* building them
/// Late compilation allows us to reuse the same pattern across languages (where a snippet will ultimately be parsed differently
/// It also allows the pattern to be used as a root pattern, or dynamically inside a function callback
pub enum UncompiledPattern {
    Contains { contains: Box<UncompiledPattern> },
    Snippet { text: String },
}
