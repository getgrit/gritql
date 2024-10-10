use crate::context::ExecContext;
use crate::pattern::ResolvedPattern;
use crate::{
    context::QueryContext,
    pattern::{Pattern, PatternOrResolved, State},
};
use grit_util::error::GritResult;
use grit_util::{error::GritPatternError, EffectKind};

#[derive(Debug, Clone)]
pub struct Effect<'a, Q: QueryContext> {
    pub binding: Q::Binding<'a>,
    pub pattern: Q::ResolvedPattern<'a>,
    pub kind: EffectKind,
}

pub fn insert_effect<'a, Q: QueryContext>(
    left: &PatternOrResolved<'a, '_, Q>,
    mut replacement: Q::ResolvedPattern<'a>,
    state: &mut State<'a, Q>,
    context: &'a Q::ExecContext<'a>,
) -> GritResult<bool> {
    match left {
        PatternOrResolved::Pattern(Pattern::Variable(var)) => {
            let var = state.trace_var_mut(var);
            if let Some(base) = state.bindings[var.try_scope().unwrap() as usize]
                .last_mut()
                .unwrap()[var.try_index().unwrap() as usize]
                .value
                .as_mut()
            {
                base.extend(replacement, &mut state.effects, context.language())?;
                Ok(true)
            } else {
                Err(GritPatternError::new(format!(
                    "Variable {} is not bound",
                    state.bindings[var.try_scope().unwrap() as usize]
                        .last()
                        .unwrap()[var.try_index().unwrap() as usize]
                        .name
                )))
            }
        }
        PatternOrResolved::Resolved(resolved) => {
            let Some(bindings) = resolved.get_bindings() else {
                return Err(
                    GritPatternError::new("variable on left hand side of insert side-conditions can only be bound to bindings")
                );
            };
            let effects: GritResult<Vec<_>> = bindings
                .map(|binding| {
                    let is_first = !state.effects.iter().any(|e| e.binding == binding);
                    replacement.normalize_insert(&binding, is_first, context.language())?;
                    Ok(Effect {
                        binding,
                        pattern: replacement.clone(),
                        kind: EffectKind::Insert,
                    })
                })
                .collect();
            let effects = effects?;
            state.effects.extend(effects);
            Ok(true)
        }
        _ => Err(GritPatternError::new(
            "Invalid left-hand side for insert operation",
        )),
    }
}
