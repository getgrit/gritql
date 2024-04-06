use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A yaml code snippet
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GritSerializedSnippet {
    /// The code snippet itself
    snippet: String,
}

/// Different kinds of patterns
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GritSerializedPattern {
    Snippet(GritSerializedSnippet),
}

// A map of metavariable -> pattern it must match
type PredicateMap = HashMap<String, GritSerializedPattern>;

/// A pattern in the yaml file
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GritSerializedPatternDefinition {
    /// Pattern
    #[serde(flatten)]
    pub pattern: GritSerializedPattern,
    /// Optional rewrite clause
    pub rewrite: Option<String>,
    /// Optional where clause
    #[serde(rename = "where")]
    pub where_clause: Option<PredicateMap>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grit_yaml_pattern_definition_deserialization() {
        let yaml = r#"
        snippet: let x = 1;
        "#;
        let deserialized: GritSerializedPatternDefinition = serde_yaml::from_str(yaml).unwrap();

        let expected = GritSerializedPatternDefinition {
            pattern: GritSerializedPattern::Snippet(GritSerializedSnippet {
                snippet: "let x = 1;".to_string(),
            }),
            rewrite: None,
            where_clause: None,
        };

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_where_clause() {
        let yaml = r#"
        snippet: let $x = 1;
        where:
          $x:
            snippet: 1
        "#;
        let deserialized: GritSerializedPatternDefinition = serde_yaml::from_str(yaml).unwrap();

        let expected = GritSerializedPatternDefinition {
            pattern: GritSerializedPattern::Snippet(GritSerializedSnippet {
                snippet: "let $x = 1;".to_string(),
            }),
            rewrite: None,
            where_clause: Some({
                let mut map = HashMap::new();
                map.insert(
                    "x".to_string(),
                    GritSerializedPattern::Snippet(GritSerializedSnippet {
                        snippet: "1".to_string(),
                    }),
                );
                map
            }),
        };

        assert_eq!(deserialized, expected);
    }
}
