use crate::{AnalysisLogs, AstNode};
use std::path::Path;

pub trait Parser {
    type Tree: Ast;

    fn parse_file(
        &mut self,
        body: &str,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
        new: bool,
    ) -> Option<Self::Tree>;

    fn parse_snippet(
        &mut self,
        pre: &'static str,
        source: &str,
        post: &'static str,
    ) -> SnippetTree<Self::Tree>;
}

pub trait Ast: Sized {
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
