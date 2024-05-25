use crate::{AnalysisLogs, AstNode};
use std::{ops::Range, path::Path};

/// Information on where a file came from, for the parser to be smarter
#[derive(Clone, Debug)]
pub enum FileOrigin<'tree, Tree>
where
    Tree: Ast,
{
    /// A file we are parsing for the first time, from disk
    Fresh,
    /// A file we have parsed before, and are re-parsing after mutating
    Mutated((&'tree Tree, &'tree Vec<(Range<usize>, usize)>)),
    /// A file that was constructed by Grit
    New,
}

impl<'tree, Tree: Ast> FileOrigin<'tree, Tree> {
    /// Is this a file we are parsing for the first time, from outside Grit?
    pub fn is_fresh(&self) -> bool {
        matches!(self, FileOrigin::Fresh)
    }
}

pub trait Parser {
    type Tree: Ast;

    fn parse_file(
        &mut self,
        body: &str,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
        origin: FileOrigin<Self::Tree>,
    ) -> Option<Self::Tree>;

    fn parse_snippet(
        &mut self,
        pre: &'static str,
        source: &str,
        post: &'static str,
    ) -> SnippetTree<Self::Tree>;
}

pub trait Ast: std::fmt::Debug + PartialEq + Sized {
    type Node<'a>: AstNode
    where
        Self: 'a;

    fn root_node(&self) -> Self::Node<'_>;
}

#[derive(Clone, Debug)]
pub struct SnippetTree<Tree: Ast> {
    pub tree: Tree,
    pub source: String,
    pub prefix: &'static str,
    pub postfix: &'static str,
    pub snippet_start: u32,
    pub snippet_end: u32,
}
