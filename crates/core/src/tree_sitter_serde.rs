use marzano_language::{
    grit_ts_node::GRIT_NODE_TYPES,
    language::{Field, Language},
};
use serde_json::{json, Value};
use tree_sitter::Node;

/**
 * Converts a tree-sitter node to a Serde JSON value.
 */
// Todo language should not be an optional
// node types should be it's own trait that Language implements
pub fn tree_sitter_node_to_json(
    node: &Node,
    source: &str,
    language: Option<&impl Language>,
) -> serde_json::Value {
    let sort_id = node.kind_id();
    let node_types = if let Some(language) = language {
        language.node_types()
    } else {
        &GRIT_NODE_TYPES
    };
    let empty: Vec<Field> = vec![];
    let node_fields = if node.is_error() {
        &empty
    } else {
        &node_types[sort_id as usize]
    };
    let mut cursor = node.walk();
    let mut node_json = json!({});
    node_json["SORT"] = node.kind().to_string().into();
    let start_position = json!({
        "line": node.start_position().row() + 1,
        "column": node.start_position().column() + 1,
    });
    let end_position = json!({
        "line": node.end_position().row() + 1,
        "column": node.end_position().column() + 1,
    });
    node_json["RANGE"] = json!({
        "start": start_position,
        "end": end_position,
    });

    if node_fields.is_empty() {
        node_json["TEXT"] = node.utf8_text(source.as_bytes()).unwrap().into();
    }

    let mut processed_child_node_ids: Vec<usize> = vec![];
    for field in node_fields {
        let children_for_field: Vec<Node> =
            node.children_by_field_id(field.id(), &mut cursor).collect();
        if children_for_field.is_empty() {
            continue;
        }
        let child_json = if field.multiple() {
            let mut children_json = vec![];
            for child in children_for_field {
                processed_child_node_ids.push(child.id());
                let child_json = tree_sitter_node_to_json(&child, source, language);
                children_json.push(child_json);
            }
            serde_json::Value::Array(children_json)
        } else {
            assert!(children_for_field.len() == 1);
            let child = &children_for_field[0];
            processed_child_node_ids.push(child.id());
            tree_sitter_node_to_json(child, source, language)
        };
        node_json[field.name()] = child_json;
    }
    let mut unprocessed_child_node: Vec<Value> = vec![];
    for child in node.children(&mut cursor) {
        if !processed_child_node_ids.contains(&child.id()) {
            let child_json = tree_sitter_node_to_json(&child, source, language);
            unprocessed_child_node.push(child_json);
        }
    }
    if !unprocessed_child_node.is_empty() {
        node_json["CHILDREN"] = serde_json::Value::Array(unprocessed_child_node);
    }
    node_json
}
