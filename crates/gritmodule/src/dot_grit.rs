use std::collections::HashMap;

use crate::{
    config::{
        DefinitionKind, GritDefinitionConfig, GritPatternMetadata, ModuleGritPattern,
        PatternVisibility,
    },
    fetcher::ModuleRepo,
    parser::extract_relative_file_path,
};
use anyhow::{anyhow, bail, Result};
use marzano_core::parse::make_grit_parser;
use marzano_language::target_language::PatternLanguage;
use marzano_util::{
    position::{map_zero_indexed_position_to_one_indexed, Position},
    rich_path::RichFile,
    tree_sitter_util::named_children_by_field_name,
};

pub fn get_patterns_from_grit(
    file: &RichFile,
    source_module: &Option<ModuleRepo>,
    root: &Option<String>,
) -> Result<Vec<ModuleGritPattern>> {
    let mut parser = make_grit_parser()?;
    let source = &file.content;
    let tree = match parser.parse(source, None)? {
        Some(tree) => tree,
        None => bail!("parse error"),
    };
    let source = source.as_bytes();
    let root_node = tree.root_node();
    let language = PatternLanguage::from_tree(&tree, &file.content);
    let mut pattern_definitions: HashMap<String, ModuleGritPattern> = HashMap::new();

    let mut cursor = root_node.walk();
    for definition in named_children_by_field_name(&root_node, &mut cursor, "definitions") {
        // when grammar is updated to remove these field we can also remove this condition
        if let Some(pattern_definition) = definition
            .child_by_field_name("pattern")
            .or_else(|| definition.child_by_field_name("predicate"))
            .or_else(|| definition.child_by_field_name("function"))
            .or_else(|| definition.child_by_field_name("foreign"))
        {
            let is_public = match pattern_definition.child_by_field_name("visibility") {
                None => true,
                Some(visibility) => {
                    let visibility = visibility.utf8_text(source)?;
                    visibility == "public"
                }
            };

            let name_node = pattern_definition
                .child_by_field_name("name")
                .ok_or_else(|| anyhow!("missing name of patternDefinition"))?;
            let name = name_node.utf8_text(source)?;
            let name = name.trim();
            let body = pattern_definition.utf8_text(source)?;
            let plain_body = body.trim();

            let kind = match pattern_definition.kind().as_ref() {
                "patternDefinition" => Some(DefinitionKind::Pattern),
                "predicateDefinition" => Some(DefinitionKind::Predicate),
                "functionDefinition" => Some(DefinitionKind::Function),
                "foreignFunctionDefinition" => Some(DefinitionKind::Function),
                _ => bail!("Bug in Grit, unhandled definition kind"),
            };

            let body = match language {
                Some(ref l) => {
                    let lang_decl = format!("language {}", l);
                    lang_decl + "\n\n" + plain_body
                }
                None => plain_body.to_string(),
            };
            let position = map_zero_indexed_position_to_one_indexed(&Position::new(
                name_node.start_position().row(),
                name_node.start_position().column(),
            ));

            let module_grit_pattern = ModuleGritPattern {
                config: GritDefinitionConfig {
                    name: name.to_string(),
                    body: Some(body.to_string()),
                    kind,
                    path: extract_relative_file_path(file, root),
                    position: Some(position),
                    meta: GritPatternMetadata {
                        tags: if is_public {
                            None
                        } else {
                            Some(vec!["hidden".to_string()])
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                visibility: if is_public {
                    PatternVisibility::Public
                } else {
                    PatternVisibility::Private
                },
                local_name: name.to_string(),
                module: source_module.clone(),
            };
            pattern_definitions.insert(name.to_string(), module_grit_pattern);
        }
    }

    Ok(pattern_definitions.into_values().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn gets_module_patterns() {
        let grit = r#"engine marzano(0.1)
language js

pattern test_identifier() {
  or {
    `describe`,
    `it`,
    `test`
  }
}

pattern no_tests_with_only() {
  `$testlike.only` => `$testlike` where {
    $testlike <: test_identifier()
  }
}

pattern no_tests_with_skip() {
  `$testlike.skip` => `$testlike` where {
    $testlike <: test_identifier()
  }
}"#;
        let repo = Default::default();
        let file = RichFile {
            content: grit.to_string(),
            path: String::new(),
        };
        let mut patterns = get_patterns_from_grit(&file, &repo, &None).unwrap();
        patterns.sort_by(|a, b| a.local_name.cmp(&b.local_name));

        assert_eq!(patterns.len(), 3);
        assert_yaml_snapshot!(patterns);
    }
}
