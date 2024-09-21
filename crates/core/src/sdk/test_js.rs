#[cfg(test)]
mod tests {
    use crate::{
        sdk::language_sdk::LanguageSdk,
        test_utils::{run_on_test_files, SyntheticFile},
    };

    #[test]
    fn test_basic_contains() {
        let sdk = LanguageSdk::default();

        let results = run_on_test_files(
            &sdk.build(sdk.snippet("console.log('hello world')").unwrap())
                .unwrap(),
            &vec![SyntheticFile::new(
                "test.js".to_owned(),
                "console.log('hello world')".to_owned(),
                true,
            )],
        );
    }
}
