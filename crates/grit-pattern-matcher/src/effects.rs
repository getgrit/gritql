use crate::context::QueryContext;
use grit_util::EffectKind;

#[derive(Debug, Clone)]
pub struct Effect<'a, Q: QueryContext> {
    pub binding: Q::Binding<'a>,
    pub pattern: Q::ResolvedPattern<'a>,
    pub kind: EffectKind,
}
