#[cfg(test)]
mod tests {
    use grit_pattern_matcher::pattern::{Contains, FilePattern, Pattern};

    use crate::{
        sdk::language_sdk::LanguageSdk,
        test_utils::{run_on_test_files, SyntheticFile},
    };

    #[test]
    fn test_basic_contains() {
        let sdk = LanguageSdk::default();

        let console = sdk.snippet("console.log('hello world')").unwrap();

        let file = Pattern::File(Box::new(FilePattern::new(
            Pattern::Top,
            Contains::new_pattern(Pattern::Top, None),
        )));

        let results = run_on_test_files(
            &sdk.build(file).unwrap(),
            &vec![SyntheticFile::new(
                "test.js".to_owned(),
                "function() {
                    console.log('hello world');
                }"
                .to_owned(),
                true,
            )],
        );
        assert_eq!(results.len(), 1);
    }
}
