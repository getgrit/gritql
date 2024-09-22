#[cfg(test)]
mod tests {
    use grit_pattern_matcher::pattern::{Contains, FilePattern, Pattern};

    use crate::{
        sdk::language_sdk::LanguageSdk,
        test_utils::{run_on_test_files, SyntheticFile},
    };

    #[test]
    fn test_basic_file_contains() {
        let sdk = LanguageSdk::default();

        let console = sdk.snippet("console.log").unwrap();

        let file = Pattern::File(Box::new(FilePattern::new(
            Pattern::Top,
            Contains::new_pattern(console, None),
        )));

        let results = run_on_test_files(
            &sdk.build(file).unwrap(),
            &[SyntheticFile::new(
                "test.js".to_owned(),
                "function() {
                    console.log('hello world');
                }"
                .to_owned(),
                true,
            )],
        );
        assert_eq!(results.len(), 2);
    }
}
