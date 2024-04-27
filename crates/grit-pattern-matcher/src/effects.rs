use crate::context::QueryContext;

#[derive(Debug, Clone)]
pub enum EffectKind {
    Rewrite,
    Insert,
}

#[derive(Debug, Clone)]
pub struct Effect<'a, Q: QueryContext> {
    pub binding: Q::Binding<'a>,
    pub pattern: Q::ResolvedPattern<'a>,
    pub kind: EffectKind,
}
