use grit_pattern_matcher::context::ExecContext;
use grit_pattern_matcher::effects::insert_effect;
use grit_pattern_matcher::pattern::PatternOrResolved;
use grit_pattern_matcher::pattern::ResolvedPattern;
use grit_pattern_matcher::pattern::State;

use grit_pattern_matcher::pattern::Variable;
use napi::{Env, Result};

use crate::marzano_context::MarzanoContext;
use crate::marzano_resolved_pattern::MarzanoResolvedPattern;
use crate::problem::MarzanoQueryContext;

/// Resolved bindings provide hooks for JavaScript code to interact with a bit of code that has been matched by a pattern.
/// The binding corresponds to a specific piece of text, which can be further filtered against or modified inside a JavaScript callback.
#[napi]
pub struct ResultBinding {
    inner: &'static MarzanoResolvedPattern<'static>,
    context: &'static MarzanoContext<'static>,
    state: &'static mut State<'static, MarzanoQueryContext>,
}

/// Internal implementation details, which we do not expose to host runtimes.
impl ResultBinding {
    pub fn new(
        inner: &'static MarzanoResolvedPattern<'static>,
        context: &'static MarzanoContext<'static>,
        state: &'static mut State<'static, MarzanoQueryContext>,
    ) -> Self {
        Self {
            inner,
            context,
            state,
        }
    }

    /// Create a new binding from pointer references
    ///
    /// SAFETY: This operation is inherently unsafe, as we cannot guarantee that the references live for the lifetime of the foreign object.
    /// If you call this, you are responsible for warning users that the underlying data cannot be accessed outside the original lifetimes.
    pub fn new_unsafe(
        binding: &MarzanoResolvedPattern,
        context: &MarzanoContext,
        state: &mut State<MarzanoQueryContext>,
    ) -> Self {
        let inner_context: &'static MarzanoContext = unsafe { std::mem::transmute(context) };
        let inner_state: &'static mut State<MarzanoQueryContext> =
            unsafe { std::mem::transmute(state) };

        let inner_binding: &'static MarzanoResolvedPattern =
            unsafe { std::mem::transmute(binding) };

        let js_binding = ResultBinding {
            inner: inner_binding,
            context: inner_context,
            state: inner_state,
        };

        js_binding
    }
}

#[napi]
impl ResultBinding {
    /// Retrieves the stringified representation of the binding (ie. the actual source code)
    #[napi]
    pub fn text(&self, _env: Env) -> Result<String> {
        let stringified_result = self
            .inner
            .text(&self.state.files, self.context.language())
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
        let string = stringified_result.to_string();
        Ok(string)
    }

    /// If the binding was found in a source file, return the position of the binding.
    #[napi]
    pub fn range(&self, _env: Env) -> Result<Option<grit_util::RangeWithoutByte>> {
        let range = self.inner.position(self.context.language());
        Ok(range.map(|e| e.into()))
    }

    /// Retrieves the absolute file name of the file containing the current binding.
    #[napi]
    pub fn filename(&self, _env: Env) -> Result<String> {
        let var = Variable::file_name();
        let resolved = var
            .get_pattern_or_resolved(self.state)
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
        let Some(candidate) = resolved else {
            return Err(napi::Error::from_reason("No resolved pattern found"));
        };
        let PatternOrResolved::Resolved(resolved) = candidate else {
            return Err(napi::Error::from_reason("No resolved pattern found"));
        };
        let stringified_result = resolved
            .text(&self.state.files, self.context.language())
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
        let string = stringified_result.to_string();
        Ok(string)
    }

    /// Inserts the provided text after the binding.
    #[napi]
    pub fn insert_after(&mut self, _env: Env, text: String) -> Result<()> {
        let left = PatternOrResolved::Resolved::<MarzanoQueryContext>(self.inner);
        let replacement = ResolvedPattern::from_string(text);

        insert_effect(&left, replacement, self.state, self.context)
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;

        Ok(())
    }
}


    /// Retrieve a variable's text value.
    // #[napi]
    // pub fn var(&self, _env: Env, name: String) -> Result<Option<String>> {
    //     let var = self.state.find_var(&name);
    //     let Some(var) = var else {
    //         return Ok(None);
    //     };
    //     let resolved = var
    //         .get_pattern_or_resolved(self.state)
    //         .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
    //     let Some(candidate) = resolved else {
    //         return Ok(None);
    //     };
    //     let PatternOrResolved::Resolved(resolved) = candidate else {
    //         return Err(napi::Error::from_reason("No resolved pattern found"));
    //     };
    //     let stringified_result = resolved
    //         .text(&self.state.files, self.context.language())
    //         .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
    //     let string = stringified_result.to_string();
    //     Ok(Some(string))
    // }
