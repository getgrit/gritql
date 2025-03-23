use grit_pattern_matcher::context::ExecContext;
use grit_pattern_matcher::effects::insert_effect;
use grit_pattern_matcher::pattern::PatternOrResolved;
use grit_pattern_matcher::pattern::ResolvedPattern;
use grit_pattern_matcher::pattern::State;
use grit_pattern_matcher::pattern::Variable;

use marzano_core::{
    marzano_context::MarzanoContext, marzano_resolved_pattern::MarzanoResolvedPattern,
    problem::MarzanoQueryContext,
};
use napi::{Env, Result};

#[napi]
pub struct JsResolvedBinding {
    pub(crate) inner: &'static MarzanoResolvedPattern<'static>,
    pub(crate) context: &'static MarzanoContext<'static>,
    pub(crate) state: &'static mut State<'static, MarzanoQueryContext>,
}

#[napi]
impl JsResolvedBinding {
    /// Retrieves the stringified representation of the binding.
    ///
    /// This method accesses the binding's text representation, using the current state's files
    /// and the specified language context.
    #[napi]
    pub fn text(&self, _env: Env) -> Result<String> {
        let stringified_result = self
            .inner
            .text(&self.state.files, self.context.language())
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
        let string = stringified_result.to_string();
        Ok(string)
    }

    /// Returns the range of the binding.
    #[napi]
    pub fn range(&self, _env: Env) -> Result<Option<grit_util::RangeWithoutByte>> {
        let range = self.inner.position(self.context.language());
        Ok(range.map(|e| e.into()))
    }

    /// Retrieve a variable's text value.
    #[napi]
    pub fn var(&self, _env: Env, name: String) -> Result<Option<String>> {
        let var = self.state.find_var(&name);
        let Some(var) = var else {
            return Ok(None);
        };
        let resolved = var
            .get_pattern_or_resolved(self.state)
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
        let Some(candidate) = resolved else {
            return Ok(None);
        };
        let PatternOrResolved::Resolved(resolved) = candidate else {
            return Err(napi::Error::from_reason("No resolved pattern found"));
        };
        let stringified_result = resolved
            .text(&self.state.files, self.context.language())
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
        let string = stringified_result.to_string();
        Ok(Some(string))
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
