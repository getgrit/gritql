use serde::{Deserialize, Serialize};

/// A yaml code snippet
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GritSerializedSnippet {
    /// The code snippet itself
    snippet: String,
}

/// Different kinds of patterns
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum GritSerializedPattern {
    Snippet(GritSerializedSnippet),
}

/// A pattern in the yaml file
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GritSerializedPatternDefinition {
    /// Pattern
    #[serde(flatten)]
    pub pattern: GritSerializedPattern,
    /// Optional rewrite clause
    pub rewrite: Option<String>,
    /// Optional where clause
    pub where_clause: Option<String>,
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
}
