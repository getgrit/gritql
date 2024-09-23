#[cfg(test)]
mod tests {
    // use grit_pattern_matcher::pattern::{Contains, DynamicPattern, FilePattern, Pattern, Rewrite};
    // use insta::assert_snapshot;

    // use crate::{
    //     api::MatchResult,
    //     sdk::language_sdk::LanguageSdk,
    //     test_utils::{run_on_test_files, SyntheticFile},
    // };

    // #[test]
    // fn test_basic_file_contains() {
    //     let mut sdk = LanguageSdk::default();

    //     let console = sdk.snippet("console.log").unwrap();

    //     let file = Pattern::File(Box::new(FilePattern::new(
    //         Pattern::Top,
    //         Contains::new_pattern(console, None),
    //     )));

    //     let results = run_on_test_files(
    //         &sdk.build(file).unwrap(),
    //         &[
    //             SyntheticFile::new(
    //                 "test.js".to_owned(),
    //                 "function() {
    //                 console.log('hello world');
    //             }"
    //                 .to_owned(),
    //                 true,
    //             ),
    //             SyntheticFile::new(
    //                 "bad.js".to_owned(),
    //                 "function() {
    //                 // no match here
    //             }"
    //                 .to_owned(),
    //                 true,
    //             ),
    //         ],
    //     );
    //     assert_eq!(results.len(), 3);
    // }

    // #[test]
    // fn test_basic_contains() {
    //     let mut sdk = LanguageSdk::default();

    //     let console = sdk.snippet("console.log").unwrap();

    //     let file = Contains::new_pattern(console, None);

    //     let results = run_on_test_files(
    //         &sdk.build(file).unwrap(),
    //         &[
    //             SyntheticFile::new(
    //                 "test.js".to_owned(),
    //                 "function() {
    //                 console.log('hello world');
    //             }"
    //                 .to_owned(),
    //                 true,
    //             ),
    //             SyntheticFile::new(
    //                 "bad.js".to_owned(),
    //                 "function() {
    //                 // no match here
    //             }"
    //                 .to_owned(),
    //                 true,
    //             ),
    //         ],
    //     );
    //     println!("{:?}", results);
    //     assert_eq!(results.len(), 3);
    // }

    // #[test]
    // fn test_basic_snippet() {
    //     let mut sdk = LanguageSdk::default();

    //     let results = run_on_test_files(
    //         &sdk.build(sdk.snippet("console.log").unwrap()).unwrap(),
    //         &[
    //             SyntheticFile::new(
    //                 "test.js".to_owned(),
    //                 "function() {
    //                 console.log('hello world');
    //             }"
    //                 .to_owned(),
    //                 true,
    //             ),
    //             SyntheticFile::new(
    //                 "bad.js".to_owned(),
    //                 "function() {
    //                 // no match here
    //             }"
    //                 .to_owned(),
    //                 true,
    //             ),
    //         ],
    //     );
    //     println!("{:?}", results);
    //     assert_eq!(results.len(), 3);
    // }

    // #[test]
    // fn test_basic_snippet_with_rewrite() {
    //     let mut sdk = LanguageSdk::default();

    //     let snippet = sdk.snippet("console.log").unwrap();

    //     let rewrite = Rewrite::new_pattern(
    //         snippet,
    //         DynamicPattern::from_str_constant("replaced").unwrap(),
    //         None,
    //     );

    //     let results = run_on_test_files(
    //         &sdk.build(rewrite).unwrap(),
    //         &[
    //             SyntheticFile::new(
    //                 "test.js".to_owned(),
    //                 "function() {
    //                 console.log('hello world');
    //                 console.log('message two');
    //             }"
    //                 .to_owned(),
    //                 true,
    //             ),
    //             SyntheticFile::new(
    //                 "bad.js".to_owned(),
    //                 "function() {
    //                 // no match here
    //             }"
    //                 .to_owned(),
    //                 true,
    //             ),
    //         ],
    //     );
    //     println!("{:?}", results);
    //     assert_eq!(results.len(), 3);
    //     let mut found_rewrite = false;
    //     for result in results {
    //         if let MatchResult::Rewrite(rewrite) = result {
    //             found_rewrite = true;
    //             assert_snapshot!(rewrite.rewritten.content.as_deref().unwrap());
    //         }
    //     }
    //     assert!(found_rewrite);
    // }

    // #[test]
    // fn test_basic_snippet_with_rewrite_and_bubble() {
    //     let mut sdk = LanguageSdk::default();

    //     let snippet = sdk.snippet("console.log($msg)").unwrap();

    //     let rewrite = Rewrite::new_pattern(
    //         snippet,
    //         DynamicPattern::from_str_constant("replaced").unwrap(),
    //         None,
    //     );

    //     let pattern = &sdk.build(rewrite).unwrap();

    //     let results = run_on_test_files(
    //         pattern,
    //         &[
    //             SyntheticFile::new(
    //                 "test.js".to_owned(),
    //                 "function() {
    //                 console.log('hello world');
    //                 console.log('message two');
    //             }"
    //                 .to_owned(),
    //                 true,
    //             ),
    //             SyntheticFile::new(
    //                 "bad.js".to_owned(),
    //                 "function() {
    //                 // no match here
    //             }"
    //                 .to_owned(),
    //                 true,
    //             ),
    //         ],
    //     );
    //     println!("{:?}", results);
    //     assert_eq!(results.len(), 3);
    //     let mut found_rewrite = false;
    //     for result in results {
    //         if let MatchResult::Rewrite(rewrite) = result {
    //             found_rewrite = true;
    //             assert_snapshot!(rewrite.rewritten.content.as_deref().unwrap());
    //         }
    //     }
    //     assert!(found_rewrite);
    // }
}
