use marzano_core::parse::make_grit_parser;
use marzano_core::pattern::analysis::defines_itself;
use marzano_util::position::{Position, Range};
use marzano_util::rich_path::RichFile;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use std::path::Path;
use tokio::io::SeekFrom;

use crate::config::{DefinitionKind, GritPatternMetadata};
use crate::{
    config::{GritDefinitionConfig, GritPatternSample, ModuleGritPattern},
    fetcher::ModuleRepo,
    parser::extract_relative_file_path,
};

use anyhow::{anyhow, bail, Result};
use markdown::{mdast::Node, to_mdast, ParseOptions};
use marzano_core::pattern::api::EnforcementLevel;

fn parse_md_snippet(tree: &Node) -> Option<&Node> {
    tree.children().unwrap().iter().find(|child| match child {
        Node::Code(n) => match &n.lang {
            Some(lang) => lang == "grit",
            None => false,
        },
        _ => false,
    })
}

fn parse_metadata(tree: &Node) -> Option<GritPatternMetadata> {
    let metadata_node = tree.children().unwrap().iter().find(|child| match child {
        Node::Heading(_) => serde_yaml::from_str::<GritPatternMetadata>(&child.to_string()).is_ok(),
        _ => false,
    });

    metadata_node?;

    match serde_yaml::from_str::<GritPatternMetadata>(&metadata_node.unwrap().to_string()) {
        Ok(frontmatter) => Some(frontmatter),
        Err(_) => None,
    }
}

fn parse_title(tree: &Node, source_lines: &[&str]) -> Option<String> {
    let first_child = tree.children().unwrap().first();
    match first_child {
        Some(Node::Heading(heading)) => {
            if heading.depth == 1 {
                let title = get_text_from_lines(first_child.unwrap(), source_lines)?;
                Some(title.trim_start_matches('#').trim().to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_md_tags(source_lines: &[&str]) -> Option<Vec<String>> {
    let regex = regex::Regex::new(r"tags:\s*(#[\w-]+,?\s*)+").unwrap();

    for line in source_lines {
        if let Some(captures) = regex.captures(line) {
            let tag_match = captures.get(0).map_or("", |m| m.as_str());
            let tag_regex = regex::Regex::new(r"#([\w-]+)").unwrap();
            let tags: Vec<String> = tag_regex
                .captures_iter(tag_match)
                .filter_map(|cap| cap.get(1))
                .map(|m| m.as_str().to_string())
                .collect();
            return Some(tags);
        }
    }

    None
}

fn parse_description(tree: &Node, source_lines: &[&str]) -> Option<String> {
    let heading_index = tree
        .children()
        .unwrap()
        .iter()
        .position(|child| match child {
            Node::Heading(n) => n.depth <= 2,
            _ => false,
        });

    heading_index?;

    let heading_index = heading_index.unwrap();
    let paragraph = tree
        .children()
        .unwrap()
        .iter()
        .skip(heading_index + 1)
        .find(|child| matches!(child, Node::Paragraph(_)));

    paragraph?;

    let paragraph = paragraph.unwrap();
    get_text_from_lines(paragraph, source_lines)
}

fn get_text_from_lines(node: &Node, source_lines: &[&str]) -> Option<String> {
    let start_line = node.position()?.start.line;
    let end_line = node.position()?.end.line;

    let lines = source_lines
        .iter()
        .skip(start_line - 1)
        .take(end_line - start_line + 1)
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    Some(lines.join("\n"))
}

struct _GritSampleSubheading<'a> {
    _node: &'a Node,
    index: usize,
    content: String,
}

fn parse_samples(tree: &Node, source_lines: &[&str]) -> Result<Vec<GritPatternSample>> {
    let samples: Vec<GritPatternSample> = Vec::new();
    let snippet = match parse_md_snippet(tree) {
        Some(snippet) => snippet,
        None => return Ok(samples),
    };

    let snippet_index = tree
        .children()
        .unwrap()
        .iter()
        .position(|node| *node == *snippet)
        .unwrap();

    let children = match tree.children() {
        Some(children) => children,
        None => return Ok(samples),
    };
    let config_samples = children.iter().skip(snippet_index).collect::<Vec<&Node>>();

    let mut samples = Vec::new();
    let mut current_subheading: Option<String> = None;
    let mut current_input: Option<String> = None;
    let mut current_input_range: Option<Range> = None;

    for node in config_samples.iter() {
        if let (Some(subheading), Some(input), Some(input_range)) =
            (&current_subheading, &current_input, &current_input_range)
        {
            match node {
                Node::Heading(_) => {
                    let sample = GritPatternSample {
                        name: Some(subheading.to_string()),
                        input: input.to_string(),
                        output: input.to_string(),
                        input_range: Some(*input_range),
                        output_range: Some(*input_range),
                    };
                    samples.push(sample);
                    current_input = None;
                    current_input_range = None;
                }
                Node::Code(n) => {
                    let output_range = n.position.as_ref().map(|position| {
                        Range::from_md(
                            // We want the internals, not the fence
                            position.start.line + 1,
                            position.start.column,
                            position.end.line - 1,
                            position.end.column,
                            position.start.offset,
                            position.end.offset,
                        )
                    });
                    let sample = GritPatternSample {
                        name: Some(subheading.to_string()),
                        input: input.to_string(),
                        output: n.value.to_string(),
                        input_range: Some(*input_range),
                        output_range,
                    };
                    samples.push(sample);
                    current_subheading = None;
                    current_input = None;
                    current_input_range = None;
                }
                _ => {}
            }
        }

        if let Node::Heading(heading) = node {
            if heading.depth == 1 {
                break;
            }
        }

        match node {
            Node::Heading(_) => {
                let content_line = match node.position() {
                    Some(position) => position.start.line,
                    None => {
                        continue;
                    }
                };
                let content = source_lines
                    .get(content_line - 1)
                    .unwrap()
                    .trim_start_matches('#')
                    .trim();
                current_subheading = Some(content.to_string());
                current_input = None;
                current_input_range = None;
            }
            Node::Code(n) if current_subheading.is_some() => {
                if current_input.is_none() {
                    current_input = Some(n.value.to_string());
                    match &n.position {
                        Some(position) => {
                            current_input_range = Some(Range::from_md(
                                // We want the internals, not the fence
                                position.start.line + 1,
                                position.start.column,
                                position.end.line - 1,
                                position.end.column,
                                position.start.offset,
                                position.end.offset,
                            ));
                        }
                        None => {}
                    }
                }
            }
            _ => {}
        }
    }

    if let (Some(subheading), Some(input), Some(input_range)) =
        (current_subheading, current_input, current_input_range)
    {
        let sample = GritPatternSample {
            name: Some(subheading),
            output: input.to_string(),
            input,
            input_range: Some(input_range),
            output_range: None,
        };
        samples.push(sample);
    }

    Ok(samples)
}

pub fn get_body_from_md_content(content: &str) -> Result<String> {
    let tree = match to_mdast(content, &ParseOptions::default()) {
        Ok(tree) => tree,
        Err(e) => bail!("Failed to parse markdown source: {}", e),
    };

    let snippet = parse_md_snippet(&tree);
    if snippet.is_none() {
        bail!("No grit body found in markdown file. Try adding a fenced code block with the language set to grit, for example:
```grit
engine marzano(0.1)
language js

js\"hello world\"
```");
    }

    let body = snippet.unwrap().to_string();

    Ok(body)
}

pub fn replace_sample_in_md_file(sample: &GritPatternSample, file_path: &String) -> Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let content_clone = content.clone();
    let source_lines: Vec<&str> = content_clone.split('\n').collect();

    let tree = match to_mdast(&content, &ParseOptions::default()) {
        Ok(tree) => tree,
        Err(e) => bail!("Failed to parse markdown source: {}", e),
    };

    let snippet = match parse_md_snippet(&tree) {
        Some(snippet) => snippet,
        None => bail!("Failed to parse markdown source"),
    };

    let snippet_index = tree
        .children()
        .unwrap()
        .iter()
        .position(|node| *node == *snippet)
        .unwrap();

    let children = match tree.children() {
        Some(children) => children,
        None => bail!("Failed to parse markdown source"),
    };

    let config_samples = children.iter().skip(snippet_index).collect::<Vec<&Node>>();

    let mut found_subheading = false;
    let mut code_blocks_replaced = 0;

    for node in config_samples.iter() {
        match node {
            Node::Heading(heading) if heading.depth == 2 => {
                let content_line = match node.position() {
                    Some(position) => position.start.line,
                    None => {
                        continue;
                    }
                };
                let heading_content = source_lines
                    .get(content_line - 1)
                    .unwrap()
                    .trim_start_matches('#')
                    .trim();

                if let Some(sample_name) = &sample.name {
                    if heading_content == sample_name.as_str() {
                        found_subheading = true;
                    }
                }
            }
            Node::Code(code) if found_subheading => {
                code_blocks_replaced += 1;
                if code_blocks_replaced == 1 {
                    let start_line = match &code.position {
                        Some(position) => position.start.line,
                        None => bail!("Failed to get start line"),
                    };
                    let end_line = match &code.position {
                        Some(position) => position.end.line,
                        None => bail!("Failed to get end line"),
                    };
                    let start = source_lines[..start_line].join("\n").len() + 1;
                    let end = source_lines[..(end_line - 1)].join("\n").len();
                    content.replace_range(start..end, &sample.input);
                } else if code_blocks_replaced == 2 {
                    let start_line = match &code.position {
                        Some(position) => position.start.line,
                        None => bail!("Failed to get start line"),
                    };
                    let end_line = match &code.position {
                        Some(position) => position.end.line,
                        None => bail!("Failed to get end line"),
                    };
                    let start = source_lines[..start_line].join("\n").len() + 1;
                    let end = source_lines[..(end_line - 1)].join("\n").len();
                    content.replace_range(start..end, &sample.output);
                    found_subheading = false;
                }
            }
            _ => {}
        }
    }

    file.seek(SeekFrom::Start(0))?;
    file.write_all(content.as_bytes())?;
    file.set_len(content.len() as u64)?;

    Ok(())
}

pub fn get_patterns_from_md(
    file: &RichFile,
    source_module: &Option<ModuleRepo>,
    root: &Option<String>,
) -> Result<Vec<ModuleGritPattern>> {
    let source = &file.content;
    let tree = match to_mdast(source, &ParseOptions::default()) {
        Ok(tree) => tree,
        Err(e) => bail!("Failed to parse markdown source: {}", e),
    };

    let snippet = parse_md_snippet(&tree);
    if snippet.is_none() {
        bail!("No grit fenced code block found in markdown file",)
    }
    let snippet = snippet.unwrap();

    let source_lines: Vec<&str> = file.content.split('\n').collect();
    let body = snippet.to_string();
    let position = snippet
        .position()
        .map(|position| Position::new(position.start.line as u32 + 1, 1));
    let path = Path::new(&file.path);
    let name = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or_else(|| file.path.trim_end_matches(".md"));

    let mut grit_parser = make_grit_parser()?;
    let src_tree = grit_parser
        .parse(&body, None)?
        .ok_or_else(|| anyhow!("parse error"))?;

    if defines_itself(&src_tree.root_node(), &body, name)? {
        bail!("Pattern {} attempts to define itself - this is not allowed. Tip: Markdown patterns use the file name as their pattern name.", name);
    }

    let samples = match parse_samples(&tree, &source_lines) {
        Ok(samples) => samples,
        Err(e) => bail!("Failed to parse samples: {}", e),
    };

    let mut meta = parse_metadata(&tree).unwrap_or_default();
    if meta.title.is_none() {
        meta.title = parse_title(&tree, &source_lines);
    }

    if meta.description.is_none() {
        meta.description = parse_description(&tree, &source_lines);
    }

    if meta.tags.is_none() {
        meta.tags = parse_md_tags(&source_lines);
    }

    if meta.level.is_none() {
        meta.level = Some(EnforcementLevel::Info);
    }

    let grit_pattern = ModuleGritPattern {
        config: GritDefinitionConfig {
            name: name.to_string(),
            body: Some(body),
            samples: Some(samples),
            kind: Some(DefinitionKind::Pattern),
            path: extract_relative_file_path(file, root),
            position,
            meta,
        },
        module: source_module.clone(),
        local_name: name.to_string(),
        ..Default::default()
    };

    Ok(vec![grit_pattern])
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

```

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
