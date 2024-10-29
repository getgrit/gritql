use crate::{
    marzano_context::MarzanoContext, marzano_resolved_pattern::MarzanoResolvedPattern,
    problem::MarzanoQueryContext,
};
use grit_pattern_matcher::{
    binding::Binding,
    context::ExecContext,
    pattern::{CodeSnippet, DynamicPattern, Matcher, Pattern, PatternName, ResolvedPattern, State},
};
use grit_util::{error::GritResult, AnalysisLogs};
use marzano_language::language::SortId;

#[derive(Debug, Clone)]
pub struct MarzanoCodeSnippet {
    pub(crate) patterns: Vec<(SortId, Pattern<MarzanoQueryContext>)>,
    pub(crate) source: String,
    pub(crate) dynamic_snippet: Option<DynamicPattern<MarzanoQueryContext>>,
}

impl MarzanoCodeSnippet {
    pub fn new(
        patterns: Vec<(SortId, Pattern<MarzanoQueryContext>)>,
        dynamic_snippet: Option<DynamicPattern<MarzanoQueryContext>>,
        source: &str,
    ) -> Self {
        Self {
            patterns,
            source: source.to_string(),
            dynamic_snippet,
        }
    }
}

impl CodeSnippet<MarzanoQueryContext> for MarzanoCodeSnippet {
    fn patterns(&self) -> impl Iterator<Item = &Pattern<MarzanoQueryContext>> {
        self.patterns.iter().map(|p| &p.1)
    }

    fn dynamic_snippet(&self) -> Option<&DynamicPattern<MarzanoQueryContext>> {
        self.dynamic_snippet.as_ref()
    }
}

impl PatternName for MarzanoCodeSnippet {
    fn name(&self) -> &'static str {
        "CODESNIPPET"
    }
}

impl Matcher<MarzanoQueryContext> for MarzanoCodeSnippet {
    // wrong, but whatever for now
    fn execute<'a>(
        &'a self,
        resolved: &MarzanoResolvedPattern<'a>,
        state: &mut State<'a, MarzanoQueryContext>,
        context: &'a MarzanoContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        let Some(binding) = resolved.get_last_binding() else {
            return Ok(resolved.text(&state.files, context.language())?.trim() == self.source);
        };

        let Some(node) = binding.singleton() else {
            return Ok(false);
        };

        if let Some((_, pattern)) = self.patterns.iter().find(|(id, p)| {
            let kind_id = node.node.kind_id();
            if *id == kind_id {
                return true;
            }
            // use equivalence classes to match 'ubuntu-latest' and "ubuntu-latest" in yaml
            // i.e. to match string_scalar, single_quote_scalar, and double_quote_scalar
            // see https://github.com/getgrit/gritql/issues/394
            match p {
                Pattern::AstLeafNode(p) => p.is_equivalent_class(kind_id),
                Pattern::AstNode(_) => false,
                Pattern::Some(_) => false,
                Pattern::Every(_) => false,
                Pattern::List(_) => false,
                Pattern::ListIndex(_) => false,
                Pattern::Map(_) => false,
                Pattern::Accessor(_) => false,
                Pattern::Call(_) => false,
                Pattern::Regex(_) => false,
                Pattern::File(_) => false,
                Pattern::Files(_) => false,
                Pattern::Bubble(_) => false,
                Pattern::Limit(_) => false,
                Pattern::CallBuiltIn(_) => false,
                Pattern::CallFunction(_) => false,
                Pattern::CallForeignFunction(_) => false,
                Pattern::CallbackPattern(_) => false,
                Pattern::Assignment(_) => false,
                Pattern::Accumulate(_) => false,
                Pattern::StringConstant(_) => false,
                Pattern::IntConstant(_) => false,
                Pattern::FloatConstant(_) => false,
                Pattern::BooleanConstant(_) => false,
                Pattern::Variable(_) => false,
                Pattern::Add(_) => false,
                Pattern::Subtract(_) => false,
                Pattern::Multiply(_) => false,
                Pattern::Divide(_) => false,
                Pattern::Modulo(_) => false,
                Pattern::And(_) => false,
                Pattern::Or(_) => false,
                Pattern::Maybe(_) => false,
                Pattern::Any(_) => false,
                Pattern::CodeSnippet(_) => false,
                Pattern::Rewrite(_) => false,
                Pattern::Range(_) => false,
                Pattern::Contains(_) => false,
                Pattern::Includes(_) => false,
                Pattern::Within(_) => false,
                Pattern::After(_) => false,
                Pattern::Before(_) => false,
                Pattern::Where(_) => false,
                Pattern::Undefined => false,
                Pattern::Top => false,
                Pattern::Underscore => false,
                Pattern::Bottom => false,
                Pattern::Not(_) => false,
                Pattern::If(_) => false,
                Pattern::Dots => false,
                Pattern::Dynamic(_) => false,
                Pattern::Sequential(_) => false,
                Pattern::Like(_) => false,
            }
        }) {
            pattern.execute(resolved, state, context, logs)
        } else {
            Ok(false)
        }
    }
}
