use crate::language::{fields_for_nodes, Field, MarzanoLanguage, NodeTypes, SortId, TSLanguage};
use grit_util::Language;
use marzano_util::node_with_source::NodeWithSource;
use std::sync::OnceLock;

static NODE_TYPES_STRING: &str = include_str!("../../../resources/node-types/csharp-node-types.json");
static NODE_TYPES: OnceLock<Vec<Vec<Field>>> = OnceLock::new();
static LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();

#[cfg(not(feature = "builtin-parser"))]
fn language() -> TSLanguage {
    unimplemented!(
        "tree-sitter parser must be initialized before use when [builtin-parser] is off."
    )
}

#[cfg(feature = "builtin-parser")]
fn language() -> TSLanguage {
    tree_sitter_c_sharp::language().into()
}

#[derive(Debug, Clone, Copy)]
pub struct CSharp {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    comment_sort: SortId,
    language: &'static TSLanguage,
}

impl CSharp {
    pub(crate) fn new(lang: Option<TSLanguage>) -> Self {
        let language = LANGUAGE.get_or_init(|| lang.unwrap_or_else(language));
        let node_types = NODE_TYPES.get_or_init(|| fields_for_nodes(language, NODE_TYPES_STRING));
        let metavariable_sort = language.id_for_node_kind("grit_metavariable", true);
        let comment_sort = language.id_for_node_kind("comment", true);
        Self {
            node_types,
            metavariable_sort,
            comment_sort,
            language,
        }
    }

    pub(crate) fn is_initialized() -> bool {
        LANGUAGE.get().is_some()
    }
}

impl NodeTypes for CSharp {
    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }
}

impl Language for CSharp {
    use_marzano_delegate!();

    fn language_name(&self) -> &'static str {
        "CSharp"
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[
            ("", ""),
            ("class Program { void Method() { ", " } }"),
            ("class Program { void Method() { ", "; } }"),
            ("class Program { ", " }"),
            ("class Program { object Method() { return ", "; } }"),
            ("class Program { void Method() { if (", ") { } } }"),
            ("class Program { var x = ", "; }"),
            ("class Program { Action a = ", "; }"),
            ("class Program { IEnumerable<object> Method() { return ", "; } }"),
        ]
    }
}

impl<'a> MarzanoLanguage<'a> for CSharp {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn is_comment_sort(&self, id: SortId) -> bool {
        id == self.comment_sort
    }

    fn metavariable_sort(&self) -> SortId {
        self.metavariable_sort
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::nodes_from_indices;

    #[test]
    fn method_call_snippet() {
        let snippet = "$obj.$method($args)";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn using_statement_snippet() {
        let snippet = "using $namespace;";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn class_declaration_snippet() {
        let snippet = "public class $name : $baseClass { }";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn method_with_attributes_snippet() {
        let snippet = "[$attribute] public async Task<$returnType> $name($params) { }";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn namespace_declaration_snippet() {
        let snippet = "namespace $name { }";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn property_accessor_snippet() {
        let snippet = "public $type $name { get => $expr; set => $field = value; }";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn linq_expression_snippet() {
        let snippet = "from $item in $collection where $condition select $result";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn async_await_snippet() {
        let snippet = "await $obj.$method($args)";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn generic_type_declaration_snippet() {
        let snippet = "public class $name<$T> where $T : $constraint { }";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn attribute_declaration_snippet() {
        let snippet = "[System.AttributeUsage($target)] public class $nameAttribute : Attribute { }";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn interface_declaration_snippet() {
        let snippet = "public interface $name { $type $method($params); }";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn lambda_expression_snippet() {
        let snippet = "($params) => $expression";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn complex_linq_query_snippet() {
        let snippet = "from $x in $collection where $x.$prop == $value group $x by $x.$key into $g select new { Key = $g.Key, Count = $g.Count() }";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn generic_method_declaration_snippet() {
        let snippet = "public $returnType $name<$T>($params) where $T : $constraint { }";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn multiple_attributes_snippet() {
        let snippet = "[$attr1, $attr2($param)] public class $name { }";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn expression_bodied_member_snippet() {
        let snippet = "public $type $name => $expression;";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn switch_expression_snippet() {
        let snippet = "$value switch { $pattern => $result, _ => $default }";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn record_declaration_snippet() {
        let snippet = "public record $name($type $prop1, $type $prop2);";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn pattern_matching_snippet() {
        let snippet = "$obj is $type";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn pattern_matching_with_variable_snippet() {
        let snippet = "$obj is $type $name";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn top_level_statements_snippet() {
        let snippet = "using $namespace; var $name = $expr;";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn init_only_property_snippet() {
        let snippet = "public $type $name { get; init; }";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn file_scoped_namespace_snippet() {
        let snippet = "namespace $name;";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn target_typed_new_snippet() {
        let snippet = "$var = new($args);";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn enhanced_pattern_matching_snippet() {
        let snippet = "$expr is not null and >= $min and <= $max";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn record_with_init_snippet() {
        let snippet = "var $var = new $record { $prop1 = $val1, $prop2 = $val2 };";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn positional_record_snippet() {
        let snippet = "public record $name($type1 $prop1, $type2 $prop2);";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn required_members_snippet() {
        let snippet = "public required $type $name { get; set; }";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn global_using_snippet() {
        let snippet = "global using $namespace;";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn extended_property_pattern_snippet() {
        let snippet = "if (obj is { $prop1: $val1, $prop2.Length: > $len })";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn raw_string_literal_snippet() {
        let snippet = "var $var = \"\"\"$content\"\"\";";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn list_pattern_snippet() {
        let snippet = "if ($list is [$first, .., $last])";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn static_abstract_interface_snippet() {
        let snippet = "public interface $name { static abstract $type $method($params); }";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn primary_constructor_snippet() {
        let snippet = "public class $name($type $param) { }";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn collection_expression_snippet() {
        let snippet = "var $var = [$item1, $item2, ..$rest];";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn constant_interpolated_string_snippet() {
        let snippet = "const string $var = $\"Hello {$name}\";";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn migration_pattern_snippet() {
        let snippet = "var $var = $obj switch { $oldPattern => $newPattern };";
        let lang = CSharp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }
}
