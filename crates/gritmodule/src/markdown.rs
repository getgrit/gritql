use grit_util::{traverse, Order};
use marzano_core::analysis::defines_itself;
use marzano_core::parse::make_grit_parser;
use marzano_language::language::Language as _;
use marzano_util::cursor_wrapper::CursorWrapper;
use marzano_util::node_with_source::NodeWithSource;
use marzano_util::position::{Position, Range};
use marzano_util::rich_path::RichFile;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use std::path::Path;
use tokio::io::SeekFrom;
use tree_sitter::Parser;

use crate::config::{DefinitionKind, GritPatternMetadata, RawGritDefinition};
use crate::{
    config::{GritDefinitionConfig, GritPatternSample, ModuleGritPattern},
    fetcher::ModuleRepo,
    parser::extract_relative_file_path,
    utils::is_pattern_name,
};
use grit_util::AstCursor as _;

use anyhow::{anyhow, bail, Context, Result};
use marzano_core::api::EnforcementLevel;

// fn parse_md_snippet(tree: &Node) -> Option<&Node> {
//     tree.children().unwrap().iter().find(|child| match child {
//         Node::Code(n) => match &n.lang {
//             Some(lang) => lang == "grit",
//             None => false,
//         },
//         _ => false,
//     })
// }

// fn parse_metadata(tree: &Node) -> Option<GritPatternMetadata> {
//     let metadata_node = tree.children().unwrap().iter().find(|child| match child {
//         Node::Heading(_) => serde_yaml::from_str::<GritPatternMetadata>(&child.to_string()).is_ok(),
//         _ => false,
//     });

//     metadata_node?;

//     match serde_yaml::from_str::<GritPatternMetadata>(&metadata_node.unwrap().to_string()) {
//         Ok(frontmatter) => Some(frontmatter),
//         Err(_) => None,
//     }
// }

// fn parse_title(tree: &Node, source_lines: &[&str]) -> Option<String> {
//     let first_child = tree.children().unwrap().first();
//     match first_child {
//         Some(Node::Heading(heading)) => {
//             if heading.depth == 1 {
//                 let title = get_text_from_lines(first_child.unwrap(), source_lines)?;
//                 Some(title.trim_start_matches('#').trim().to_string())
//             } else {
//                 None
//             }
//         }
//         _ => None,
//     }
// }

// fn parse_md_tags(source_lines: &[&str]) -> Option<Vec<String>> {
//     let regex = regex::Regex::new(r"tags:\s*(#[\w-]+,?\s*)+").unwrap();

//     for line in source_lines {
//         if let Some(captures) = regex.captures(line) {
//             let tag_match = captures.get(0).map_or("", |m| m.as_str());
//             let tag_regex = regex::Regex::new(r"#([\w-]+)").unwrap();
//             let tags: Vec<String> = tag_regex
//                 .captures_iter(tag_match)
//                 .filter_map(|cap| cap.get(1))
//                 .map(|m| m.as_str().to_string())
//                 .collect();
//             return Some(tags);
//         }
//     }

//     None
// }

// fn parse_description(tree: &Node, source_lines: &[&str]) -> Option<String> {
//     let heading_index = tree
//         .children()
//         .unwrap()
//         .iter()
//         .position(|child| match child {
//             Node::Heading(n) => n.depth <= 2,
//             _ => false,
//         });

//     heading_index?;

//     let heading_index = heading_index.unwrap();
//     let paragraph = tree
//         .children()
//         .unwrap()
//         .iter()
//         .skip(heading_index + 1)
//         .find(|child| matches!(child, Node::Paragraph(_)));

//     paragraph?;

//     let paragraph = paragraph.unwrap();
//     get_text_from_lines(paragraph, source_lines)
// }

// fn get_text_from_lines(node: &Node, source_lines: &[&str]) -> Option<String> {
//     let start_line = node.position()?.start.line;
//     let end_line = node.position()?.end.line;

//     let lines = source_lines
//         .iter()
//         .skip(start_line - 1)
//         .take(end_line - start_line + 1)
//         .filter(|line| !line.is_empty())
//         .map(|line| line.to_string())
//         .collect::<Vec<_>>();

//     Some(lines.join("\n"))
// }

/// Capture a single markdown GritQL body - there might be multiple in a single markdown file
#[derive(Debug)]
struct MarkdownBody {
    body: String,
    position: Position,
    section_level: u32,
    /// Track the samples which have already been matched together
    samples: Vec<GritPatternSample>,
    /// Leave "open" sample which have not been matched to a pair yet
    open_sample: Option<GritPatternSample>,
}

pub fn make_md_parser() -> Result<Parser> {
    use anyhow::Context;

    let mut parser = Parser::new().unwrap();
    let language = marzano_language::markdown_block::MarkdownBlock::new(None);
    parser
        .set_language(language.get_ts_language())
        .with_context(|| "Failed to load markdown grammar")?;
    Ok(parser)
}
pub fn get_patterns_from_md(
    file: &RichFile,
    source_module: &Option<ModuleRepo>,
    root: &Option<String>,
) -> Result<Vec<ModuleGritPattern>> {
    let src = &file.content;

    let path = Path::new(&file.path);
    let name = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or_else(|| file.path.trim_end_matches(".md"));
    // if !is_pattern_name(name) {
    //     bail!("Invalid pattern name: {}. Grit patterns must match the regex /[\\^#A-Za-z_][A-Za-z0-9_]*/. For more info, consult the docs at https://docs.grit.io/guides/patterns#pattern-definitions.", name);
    // }
    let relative_path = extract_relative_file_path(file, root);

    let mut parser = make_md_parser()?;
    let mut grit_parser = make_grit_parser()?;

    let src_tree = parser
        .parse(src, None)?
        .context("No valid Markdown tree found")?;
    let root_node = NodeWithSource::new(src_tree.root_node(), src);

    let mut cursor = CursorWrapper::new(root_node.node.walk(), root_node.source);

    let mut patterns: Vec<MarkdownBody> = Vec::new();

    // Track the current language block for when we hit the actual content
    let mut current_code_block_language = None;

    // Track the current heading level, used for determining which patterns should be grouped together
    let mut current_heading = (1, "".to_string());

    for n in traverse(cursor, Order::Pre) {
        if n.node.kind() == "language" {
            current_code_block_language = Some(n.node.utf8_text(src.as_bytes()).unwrap());
        } else if n.node.kind() == "code_fence_content" {
            let content = n.node.utf8_text(src.as_bytes()).unwrap();
            if current_code_block_language == Some(std::borrow::Cow::Borrowed("grit")) {
                // TODO: this bit
                // let src_tree = grit_parser
                //     .parse(content.into_owned(), None)?
                //     .ok_or_else(|| anyhow!("parse error"))?;
                // if defines_itself(&NodeWithSource::new(src_tree.root_node(), &body), name)? {
                //     bail!("Pattern {} attempts to define itself - this is not allowed. Tip: Markdown patterns use the file name as their pattern name.", name);
                // }

                let definition = MarkdownBody {
                    body: content.to_string(),
                    position: n.node.range().start_point().into(),
                    section_level: current_heading.0,
                    samples: Vec::new(),
                    open_sample: None,
                };
                patterns.push(definition);
            } else if let Some(last_config) = patterns.last_mut() {
                // Check if we have an open sample
                if let Some(mut open_sample) = last_config.open_sample.take() {
                    open_sample.output = Some(content.to_string());
                    open_sample.output_range = Some(n.node.range().into());
                    last_config.samples.push(open_sample);
                } else {
                    // Start a new sample
                    let sample = GritPatternSample {
                        name: if !current_heading.1.is_empty() {
                            Some(current_heading.1.clone())
                        } else {
                            None
                        },
                        input: content.to_string(),
                        output: None,
                        input_range: Some(n.node.range().into()),
                        output_range: None,
                    };
                    last_config.open_sample = Some(sample);
                }
            }
            current_code_block_language = None;
        } else if n.node.kind() == "atx_heading" {
            let heading_level = n.node.child_by_field_name("level").unwrap();
            let heading_level = match heading_level.kind() {
                std::borrow::Cow::Borrowed("atx_h1_marker") => 1,
                std::borrow::Cow::Borrowed("atx_h2_marker") => 2,
                std::borrow::Cow::Borrowed("atx_h3_marker") => 3,
                std::borrow::Cow::Borrowed("atx_h4_marker") => 4,
                std::borrow::Cow::Borrowed("atx_h5_marker") => 5,
                std::borrow::Cow::Borrowed("atx_h6_marker") => 6,
                _ => 6,
            };

            if let Some(last_config) = patterns.last_mut() {
                // If the grit block started in an h1, then new samples are introduced with each h3
                if heading_level <= last_config.section_level + 2 {
                    if let Some(open_sample) = last_config.open_sample.take() {
                        last_config.samples.push(open_sample);
                    }
                }
            }

            let heading_text = n
                .node
                .child_by_field_name("heading_content")
                .unwrap()
                .utf8_text(src.as_bytes())
                .unwrap()
                .to_string();
            let heading_text = heading_text.trim().to_string();

            current_heading = (heading_level, heading_text);
        }
        // println!(
        //     "Finished node: {:?} in language: {:?}, defs are {:?}",
        //     n.node.kind(),
        //     current_code_block_language,
        //     config.len()
        // );
    }

    println!("We ended up with {:?}", patterns);

    // let mut meta = parse_metadata(&tree).unwrap_or_default();
    // if meta.title.is_none() {
    //     meta.title = parse_title(&tree, &source_lines);
    // }

    // if meta.description.is_none() {
    //     meta.description = parse_description(&tree, &source_lines);
    // }

    // if meta.tags.is_none() {
    //     meta.tags = parse_md_tags(&source_lines);
    // }

    // if meta.level.is_none() {
    //     meta.level = Some(EnforcementLevel::Info);
    // }

    let patterns = patterns
        .into_iter()
        .enumerate()
        .map(|(i, p)| {
            let local_name = if i == 0 {
                name.to_string()
            } else {
                format!("{}-{}", name, i)
            };
            ModuleGritPattern {
                config: GritDefinitionConfig {
                    name: local_name.clone(),
                    body: Some(p.body),
                    meta: GritPatternMetadata::default(),
                    kind: Some(DefinitionKind::Pattern),
                    samples: if p.samples.is_empty() {
                        None
                    } else {
                        Some(p.samples)
                    },
                    path: relative_path.clone(),
                    position: Some(p.position),
                    raw: Some(RawGritDefinition {
                        content: src.to_string(),
                        format: crate::parser::PatternFileExt::Md,
                    }),
                },
                module: source_module.clone(),
                local_name,
                ..Default::default()
            }
        })
        .collect();
    Ok(patterns)
}

pub fn get_body_from_md_content(content: &str) -> Result<String> {
    let patterns = get_patterns_from_md(
        &RichFile {
            path: "test.md".to_string(),
            content: content.to_string(),
        },
        &None,
        &None,
    )?;

    if let Some(pattern) = patterns.first() {
        if let Some(body) = &pattern.config.body {
            return Ok(body.to_string());
        }
    }

    bail!("No grit body found in markdown file. Try adding a fenced code block with the language set to grit, for example:
```grit
engine marzano(0.1)
language js

js\"hello world\"
```");
}

pub fn replace_sample_in_md_file(sample: &GritPatternSample, file_path: &String) -> Result<()> {
    todo!("This needs to be implemented");
    // let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

    // let mut content = String::new();
    // file.read_to_string(&mut content)?;

    // let content_clone = content.clone();
    // let source_lines: Vec<&str> = content_clone.split('\n').collect();

    // let tree = match to_mdast(&content, &ParseOptions::default()) {
    //     Ok(tree) => tree,
    //     Err(e) => bail!("Failed to parse markdown source: {}", e),
    // };

    // let snippet = match parse_md_snippet(&tree) {
    //     Some(snippet) => snippet,
    //     None => bail!("Failed to parse markdown source"),
    // };

    // let snippet_index = tree
    //     .children()
    //     .unwrap()
    //     .iter()
    //     .position(|node| *node == *snippet)
    //     .unwrap();

    // let children = match tree.children() {
    //     Some(children) => children,
    //     None => bail!("Failed to parse markdown source"),
    // };

    // let config_samples = children.iter().skip(snippet_index).collect::<Vec<&Node>>();

    // let mut found_subheading = false;
    // let mut code_blocks_replaced = 0;

    // for node in config_samples.iter() {
    //     match node {
    //         Node::Heading(heading) if heading.depth == 2 => {
    //             let content_line = match node.position() {
    //                 Some(position) => position.start.line,
    //                 None => {
    //                     continue;
    //                 }
    //             };
    //             let heading_content = source_lines
    //                 .get(content_line - 1)
    //                 .unwrap()
    //                 .trim_start_matches('#')
    //                 .trim();

    //             if let Some(sample_name) = &sample.name {
    //                 if heading_content == sample_name.as_str() {
    //                     found_subheading = true;
    //                 }
    //             }
    //         }
    //         Node::Code(code) if found_subheading => {
    //             code_blocks_replaced += 1;
    //             if code_blocks_replaced == 1 {
    //                 let start_line = match &code.position {
    //                     Some(position) => position.start.line,
    //                     None => bail!("Failed to get start line"),
    //                 };
    //                 let end_line = match &code.position {
    //                     Some(position) => position.end.line,
    //                     None => bail!("Failed to get end line"),
    //                 };
    //                 let start = source_lines[..start_line].join("\n").len() + 1;
    //                 let end = source_lines[..(end_line - 1)].join("\n").len();
    //                 content.replace_range(start..end, &sample.input);
    //             } else if code_blocks_replaced == 2 && sample.output.is_some() {
    //                 let start_line = match &code.position {
    //                     Some(position) => position.start.line,
    //                     None => bail!("Failed to get start line"),
    //                 };
    //                 let end_line = match &code.position {
    //                     Some(position) => position.end.line,
    //                     None => bail!("Failed to get end line"),
    //                 };
    //                 let start = source_lines[..start_line].join("\n").len() + 1;
    //                 let end = source_lines[..(end_line - 1)].join("\n").len();
    //                 content.replace_range(start..end, sample.output.as_ref().unwrap());
    //                 found_subheading = false;
    //             }
    //         }
    //         _ => {}
    //     }
    // }

    // file.seek(SeekFrom::Start(0))?;
    // file.write_all(content.as_bytes())?;
    // file.set_len(content.len() as u64)?;

    // Ok(())
}

#[cfg(test)]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;

    #[test]
    fn test_markdown_parser() {
        let module = Default::default();
        let rich_file = RichFile { path: "no_debugger.md".to_string(),content: r#"---
title: Remove `debugger` statement
---

The code in production should not contain a `debugger`. It causes the browser to stop executing the code and open the debugger.

tags: #fix

```grit
engine marzano(0.1)
language js

debugger_statement() => .
```

## Remove debbuger

```javascript
function isTruthy(x) {
  debugger;
  return Boolean(x);
}
```

```typescript
function isTruthy(x) {
  return Boolean(x);
}
```
"#.to_string()};
        let patterns = get_patterns_from_md(&rich_file, &module, &None).unwrap();
        assert_eq!(patterns.len(), 1);
        println!("{:?}", patterns);
        assert_yaml_snapshot!(patterns);
    }

    #[test]
    fn test_sample_labels() {
        let module = Default::default();
        let rich_file = RichFile { path: "no_debugger.md".to_string(),content: r#"---
title: Remove `debugger` statement
---

The code in production should not contain a `debugger`. It causes the browser to stop executing the code and open the debugger.

tags: #fix

```grit
engine marzano(0.1)
language js

debugger_statement() => .
```

## Remove debbuger

### Bad

```javascript
function isTruthy(x) {
  debugger;
  return Boolean(x);
}
```

### Good

```typescript
function isTruthy(x) {
  return Boolean(x);
}
```
"#.to_string()};
        let patterns = get_patterns_from_md(&rich_file, &module, &None).unwrap();
        assert_eq!(patterns.len(), 1);
        println!("{:?}", patterns);
        assert_yaml_snapshot!(patterns);
    }

    #[test]
    fn test_multiple_markdown_patterns() {
        let module = Default::default();
        let rich_file = RichFile {
            path: "no_debugger.md".to_string(),
            content: r#"
This is a single Markdown file with multiple grit patterns.

## Pattern one

```grit
engine marzano(0.1)
language js

debugger_statement() => .
```

Example:

```javascript
function isTruthy(x) {
  debugger;
  return Boolean(x);
}
```

```typescript
function isTruthy(x) {
  return Boolean(x);
}

### Test case 2

```javascript
function isTruthy(x) {
  debugger;
  return Boolean(x);
}
```

```typescript
function isTruthy(x) {
  return Boolean(x);
}


## Pattern two

```grit
engine marzano(0.1)
language js

`console.log($_)` => .
```

### Test case 3 - non matching.

```javascript
function isTruthy(x) {
  console.error("Hello");
  return Boolean(x);
}
```

### Test case 4

```javascript
function isTruthy(x) {
  console.log("Hello");
  return Boolean(x);
}
```

```typescript
function isTruthy(x) {
  return Boolean(x);
}

```
"#
            .to_string(),
        };
        let patterns = get_patterns_from_md(&rich_file, &module, &None).unwrap();
        assert_eq!(patterns.len(), 1);
        println!("{:?}", patterns);
        assert_yaml_snapshot!(patterns);
    }

    #[test]
    fn test_frontmatter_enforcement() {
        let module = Default::default();
        let rich_file = RichFile {
            path: "no_console_log.md".to_string(),
            content: r#"---
title: Forbid console.log
level: error
---

This is bad!

```grit
engine marzano(0.1)
language js

`console.log($_)` => .
```

"#
            .to_string(),
        };
        let patterns = get_patterns_from_md(&rich_file, &module, &None).unwrap();
        assert_eq!(patterns.len(), 1);
        println!("{:?}", patterns);
        assert_eq!(patterns[0].config.meta.level, Some(EnforcementLevel::Error));
    }

    #[test]
    fn with_non_matching_samples() {
        let module = Default::default();
        let rich_file = RichFile {
            path: "no_eq_null.md".to_string(),
            content: r#"---
title: Compare `null` using  `===` or `!==`
---

Comparing to `null` needs a type-checking operator (=== or !==), to avoid incorrect results when the value is `undefined`.

tags: #good

```grit
engine marzano(0.1)
language js

// We use the syntax-tree node binary_expression to capture all expressions where $a and $b are operated on by "==" or "!=".
// This code takes advantage of Grit's allowing us to nest rewrites inside match conditions and to match syntax-tree fields on patterns.
binary_expression($operator, $left, $right) where {
    $operator <: or  { "==" => `===` , "!=" => `!==` },
    or { $left <: `null`, $right <: `null`}
}

```

```

```

## `$val == null` => `$val === null`

```javascript
if (val == null) {
  done();
}
```

```typescript
if (val === null) {
  done();
}
```

## `$val != null` => `$val !== null`

```javascript
if (val != null) {
  done();
}
```

```typescript
if (val !== null) {
  done();
}
```

## `$val != null` => `$val !== null` into `while`

```javascript
while (val != null) {
  did();
}
```

```typescript
while (val !== null) {
  did();
}
```

## Do not change `$val === null`

```javascript
if (val === null) {
  done();
}
```

## Do not change `$val !== null`

```
while (val !== null) {
  doSomething();
}
```
"#.to_string(),
        };
        let patterns = get_patterns_from_md(&rich_file, &module, &None).unwrap();
        assert_eq!(patterns.len(), 1);
        assert_yaml_snapshot!(patterns);
    }

    #[test]
    fn with_no_frontmatter() {
        let module = ModuleRepo::from_host_repo("github.com", "getgrit/rewriter").unwrap();
        let rich_file = RichFile {
            path: "CloneActivities.md".to_string(),
            content: r#"# Copy activity names to timekeeper

We cannot directly include `@getgrit/sdk` in `@getgrit/timekeeper` but we _can_ use Grit to copy over the function names list.

```grit
engine marzano(0.1)
language js

multifile {
    $names = [],
    bubble($names) file($name, $body) where {
        $name <: r".+sdk/src/stdlib/index.ts",
        $body <: contains `const stdlib = { $activities }` where {
            $activities <: some bubble($names) $activity where {
                $names += `'$activity'`
            }
        }
    },
    bubble($names) file($name, $body) where {
      $body <: contains `const stdlib = [$old] as const`,
      $name <: r".+__generated__/stdlib.ts",
      $new_names = join(list = $names, separator = ", "),
      $old => `$new_names`,
    },
}
```
"#.to_string(),};
        let patterns = get_patterns_from_md(&rich_file, &Some(module), &None).unwrap();
        assert_eq!(patterns.len(), 1);
        assert_yaml_snapshot!(patterns);
    }
}
