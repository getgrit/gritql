use marzano_language::target_language::TargetLanguage;

use self::pattern_compiler::src_to_problem_libs;

use super::*;
use std::collections::BTreeMap;

#[test]
fn test_error_invalid_field() {
    let pattern_src = r#"
        string_fragment(fragment=$d)
        "#;
    let libs = BTreeMap::new();

    let err = src_to_problem_libs(
        pattern_src.to_string(),
        &libs,
        TargetLanguage::default(),
        None,
        None,
        None,
        None,
    )
    .err()
    .unwrap();
    assert_eq!(
        err.to_string(),
        "invalid field `fragment` for AST node `string_fragment`. `string_fragment` does not expose any fields."
    );
}
