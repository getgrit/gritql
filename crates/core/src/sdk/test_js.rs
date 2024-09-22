#[cfg(test)]
mod tests {
    use grit_pattern_matcher::pattern::{Contains, FilePattern, Pattern};

    use crate::{
        sdk::language_sdk::LanguageSdk,
        test_utils::{run_on_test_files, SyntheticFile},
    };

    #[test]
    fn test_basic_file_contains() {
        let mut sdk = LanguageSdk::default();

        let console = sdk.snippet("console.log").unwrap();

        let file = Pattern::File(Box::new(FilePattern::new(
            Pattern::Top,
            Contains::new_pattern(console, None),
        )));

        let results = run_on_test_files(
            &sdk.build(file).unwrap(),
            &[
                SyntheticFile::new(
                    "test.js".to_owned(),
                    "function() {
                    console.log('hello world');
                }"
                    .to_owned(),
                    true,
                ),
                SyntheticFile::new(
                    "bad.js".to_owned(),
                    "function() {
                    // no match here
                }"
                    .to_owned(),
                    true,
                ),
            ],
        );
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_basic_contains() {
        let mut sdk = LanguageSdk::default();

        let console = sdk.snippet("console.log").unwrap();

        let file = Contains::new_pattern(console, None);

        let results = run_on_test_files(
            &sdk.build(file).unwrap(),
            &[
                SyntheticFile::new(
                    "test.js".to_owned(),
                    "function() {
                    console.log('hello world');
                }"
                    .to_owned(),
                    true,
                ),
                SyntheticFile::new(
                    "bad.js".to_owned(),
                    "function() {
                    // no match here
                }"
                    .to_owned(),
                    true,
                ),
            ],
        );
        println!("{:?}", results);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_basic_snippet() {
        let mut sdk = LanguageSdk::default();

        let results = run_on_test_files(
            &sdk.build(sdk.snippet("console.log").unwrap()).unwrap(),
            &[
                SyntheticFile::new(
                    "test.js".to_owned(),
                    "function() {
                    console.log('hello world');
                }"
                    .to_owned(),
                    true,
                ),
                SyntheticFile::new(
                    "bad.js".to_owned(),
                    "function() {
                    // no match here
                }"
                    .to_owned(),
                    true,
                ),
            ],
        );
        println!("{:?}", results);
        assert_eq!(results.len(), 3);
    }
}
