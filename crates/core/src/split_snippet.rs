use std::{borrow::Cow, ops::Range};

use marzano_language::language::Language;

/**
 * Takes a snippet with metavariables and returns
 * a list of ranges and the corresponding metavariables.
 *
 * The ranges are in descending order.
 *
 * The regex for identifying metavariables are:
 * /[\$\^#A-Za-z_][A-Za-z0-9_]/
 * \$\[[A-Za-z_][A-Za-z0-9_]*\]
 */

pub fn split_snippet<'a>(
    snippet: &'a str,
    lang: &impl Language,
) -> Vec<(Range<u32>, Cow<'a, str>)> {
    let mut ranges_and_metavars: Vec<(Range<u32>, Cow<str>)> = Vec::new();

    let variable_regex = lang.metavariable_regex();
    let curly_var_regex = lang.metavariable_bracket_regex();

    for m in variable_regex.find_iter(snippet) {
        ranges_and_metavars.push((m.start() as u32..m.end() as u32, m.as_str().into()));
    }
    for m in curly_var_regex.find_iter(snippet) {
        let mut metavar: Cow<str> = m.as_str()[2..m.as_str().len() - 1].into();
        metavar.to_mut().insert(0, '$');
        ranges_and_metavars.push((m.start() as u32..m.end() as u32, metavar));
    }

    // Sort ranges in descending order
    ranges_and_metavars.sort_by(|a, b| b.0.start.cmp(&a.0.start));

    ranges_and_metavars
}

#[cfg(test)]
mod tests {
    use marzano_language::target_language::{PatternLanguage, TargetLanguage};

    use super::*;

    #[test]
    fn test_empty_snippet() {
        let snippet = "";
        let lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
        let result = split_snippet(snippet, &lang);
        assert_eq!(result, Vec::<(Range<u32>, Cow<str>)>::new());
    }

    #[test]
    fn test_no_metavars() {
        let snippet = "This is a test snippet with no metavariables.";
        let lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
        let result = split_snippet(snippet, &lang);
        assert_eq!(result, Vec::<(Range<u32>, Cow<str>)>::new());
    }

    #[test]
    fn test_single_metavar() {
        let snippet = "This is a $test snippet.";
        let lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
        let result = split_snippet(snippet, &lang);
        let expected = vec![(10..15, "$test".into())];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_metavars() {
        let snippet = "This is a $test snippet with $multiple $metavariables.";
        let lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
        let result = split_snippet(snippet, &lang);
        let expected = vec![
            (39..53, "$metavariables".into()),
            (29..38, "$multiple".into()),
            (10..15, "$test".into()),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_adjacent_metavars() {
        let snippet = "This is a $test$example snippet.";
        let lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
        let result = split_snippet(snippet, &lang);
        let expected = vec![(15..23, "$example".into()), (10..15, "$test".into())];
        assert_eq!(result, expected);
    }
}
