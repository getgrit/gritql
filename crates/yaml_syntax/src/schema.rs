/// A yaml code snippet
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub enum GritYamlSnippet {
    /// The code snippet itself
    snippet: String,
}

/// Different kinds of patterns
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub enum GritYamlPattern {
    Snippet(GritYamlSnippet),
}

/// A pattern in the yaml file
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct GritYamlPatternDefinition {
    /// Pattern
    #[serde(flatten)]
    pub pattern: GritYamlPattern,
    /// Optional rewrite clause
    pub rewrite: Option<String>,
    /// Optional where clause
    pub where_clause: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_grit_yaml_snippet_serialization() {
        let snippet = GritYamlSnippet {
            snippet: "example_snippet".to_string(),
        };
        let serialized = serde_json::to_string(&snippet).unwrap();
        assert_eq!(serialized, r#"{"snippet":"example_snippet"}"#);
    }

    #[test]
    fn test_grit_yaml_pattern_serialization() {
        let pattern = GritYamlPattern::Snippet(GritYamlSnippet {
            snippet: "example_snippet".to_string(),
        });
        let serialized = serde_json::to_string(&pattern).unwrap();
        assert_eq!(serialized, r#"{"Snippet":{"snippet":"example_snippet"}}"#);
    }

    #[test]
    fn test_grit_yaml_pattern_definition_serialization() {
        let pattern_def = GritYamlPatternDefinition {
            pattern: GritYamlPattern::Snippet(GritYamlSnippet {
                snippet: "example_snippet".to_string(),
            }),
            rewrite: Some("example_rewrite".to_string()),
            where_clause: Some("example_where".to_string()),
        };
        let serialized = serde_json::to_string(&pattern_def).unwrap();
        let expected = json!({
            "pattern": {"Snippet": {"snippet": "example_snippet"}},
            "rewrite": "example_rewrite",
            "where_clause": "example_where"
        });
        assert_eq!(serialized, expected.to_string());
    }
}
