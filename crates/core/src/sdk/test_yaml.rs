#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use grit_pattern_matcher::pattern::{
        Contains, DynamicPattern, FilePattern, Pattern, Rewrite, StringConstant,
    };
    use insta::assert_snapshot;
    use itertools::Itertools;
    use marzano_language::target_language::TargetLanguage;

    use crate::{
        api::{FileMatchResult, MatchResult},
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

    #[test]
    fn test_yaml_patterns() {
        let mut sdk =
            LanguageSdk::from_language(TargetLanguage::from_string("yaml", None).unwrap());

        let yaml_content = r#"version: 0.0.1
patterns:
  - name: github.com/getgrit/stdlib#*
  - name: other_pattern
    level: error
    body: |
      language toml

      other_pattern() where $filename <: includes "test.yaml"
  - name: target_pattern
    level: error
    body: |
      language foolhardy
  - name: our_cargo_use_long_dependency
    level: error
    body: |
      language toml

      cargo_use_long_dependency() where $filename <: not includes or {
        "language-submodules",
        "language-metavariables"
      }"#;

        let our_name = sdk
            .compiler()
            .node(
                "block_mapping_pair",
                HashMap::from([
                    (
                        "key",
                        Pattern::StringConstant(StringConstant::new("name".to_owned())),
                    ),
                    (
                        "value",
                        Pattern::StringConstant(StringConstant::new("target_pattern".to_owned())),
                    ),
                ]),
            )
            .unwrap();
        let our_body = sdk
            .compiler()
            .node(
                "block_mapping_pair",
                HashMap::from([
                    (
                        "key",
                        Pattern::StringConstant(StringConstant::new("body".to_owned())),
                    ),
                    (
                        "value",
                        Pattern::Rewrite(Box::new(grit_pattern_matcher::pattern::Rewrite::new(
                            Pattern::Top,
                            DynamicPattern::from_str_constant("replacement body").unwrap(),
                            None,
                        ))),
                    ),
                ]),
            )
            .unwrap();
        let our_block = sdk
            .compiler()
            .node(
                "block_mapping",
                HashMap::from([(
                    "items",
                    Pattern::And(Box::new(grit_pattern_matcher::pattern::And::new(vec![
                        Pattern::Some(Box::new(grit_pattern_matcher::pattern::Some::new(our_name))),
                        Pattern::Some(Box::new(grit_pattern_matcher::pattern::Some::new(our_body))),
                    ]))),
                )]),
            )
            .unwrap();

        let file = Pattern::File(Box::new(FilePattern::new(
            Pattern::Top,
            Contains::new_pattern(our_block, None),
        )));

        let results = run_on_test_files(
            &sdk.build(file).unwrap(),
            &[
                SyntheticFile::new("test.yaml".to_owned(), yaml_content.to_owned(), true),
                SyntheticFile::new("bad.yaml".to_owned(), "no: name\n".to_owned(), true),
            ],
        );

        println!("{:?}", results);

        let rewrite = results
            .iter()
            .filter(|r| matches!(r, MatchResult::Rewrite(_)))
            .exactly_one()
            .unwrap();

        if let MatchResult::Rewrite(content) = rewrite {
            let after = content.content().unwrap();
            assert_snapshot!("rewrite", after);
        }
    }
}
