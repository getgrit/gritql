#[cfg(test)]
mod tests {
    use grit_pattern_matcher::pattern::{Contains, DynamicPattern, FilePattern, Pattern, Rewrite};
    use insta::assert_snapshot;
    use itertools::Itertools;
    use marzano_language::target_language::TargetLanguage;

    use crate::{
        api::MatchResult,
        sdk::language_sdk::LanguageSdk,
        test_utils::{run_on_test_files, SyntheticFile},
    };

    #[test]
    fn test_basic_file_contains() {
        let mut sdk =
            LanguageSdk::from_language(TargetLanguage::from_string("yaml", None).unwrap());

        let console = sdk.snippet("foo: bar").unwrap();

        let file = Pattern::File(Box::new(FilePattern::new(
            Pattern::Top,
            Contains::new_pattern(console, None),
        )));

        let results = run_on_test_files(
            &sdk.build(file).unwrap(),
            &[
                SyntheticFile::new(
                    "test.yaml".to_owned(),
                    "stuff:
  bar:
    baz:
      qux: quux
      foo: bar
      corge:
        - grault
        - garply
      waldo:
        fred: plugh
        xyzzy: thud\n"
                        .to_owned(),
                    true,
                ),
                SyntheticFile::new("bad.yaml".to_owned(), "bar: baz\n".to_owned(), true),
            ],
        );

        assert!(
            results
                .iter()
                .filter(|r| matches!(r, MatchResult::Match(_)))
                .exactly_one()
                .is_ok(),
            "should have exactly one match"
        );
    }
}
