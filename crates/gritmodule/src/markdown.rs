use grit_util::{traverse, Order};
use marzano_core::analysis::defines_itself;
use marzano_core::parse::make_grit_parser;
use marzano_language::language::Language as _;
use marzano_util::cursor_wrapper::CursorWrapper;
use marzano_util::node_with_source::NodeWithSource;
use marzano_util::position::Position;
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

use anyhow::{anyhow, bail, Context, Result};
use marzano_core::api::EnforcementLevel;

fn parse_metadata(yaml_content: &str) -> Result<GritPatternMetadata> {
    let result = serde_yaml::from_str::<GritPatternMetadata>(yaml_content)?;
    Ok(result)
}

/// Capture a single markdown GritQL body - there might be multiple in a single markdown file
#[derive(Debug)]
struct MarkdownBody {
    body: String,
    position: Position,
    section_heading: String,
    section_level: u32,
    /// Track the samples which have already been matched together
    samples: Vec<GritPatternSample>,
    /// Leave "open" sample which have not been matched to a pair yet
    open_sample: Option<GritPatternSample>,
}

pub fn make_md_parser() -> Result<Parser> {
    let mut parser = Parser::new().unwrap();
    let language = marzano_language::markdown_block::MarkdownBlock::new(None);
    parser
        .set_language(language.get_ts_language())
        .with_context(|| "Failed to load markdown grammar")?;
    Ok(parser)
}
pub fn get_patterns_from_md(
    file: &mut RichFile,
    source_module: &Option<ModuleRepo>,
    root: &Option<String>,
) -> Result<Vec<ModuleGritPattern>> {
    // Tree sitter has weird problems if we are missing a newline at the end of the file
    if !file.content.ends_with('\n') {
        file.content.push('\n');
    }

    let src = &file.content;

    let path = Path::new(&file.path);
    let name = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or_else(|| file.path.trim_end_matches(".md"));
    if !is_pattern_name(name) {
        bail!("Invalid pattern name: '{}'. Grit patterns must match the regex /^[A-Za-z_][A-Za-z0-9_]*$/. For more info, consult the docs at https://docs.grit.io/guides/patterns#pattern-definitions.", name);
    }

    let relative_path = extract_relative_file_path(file, root);

    let mut parser = make_md_parser()?;

    let src_tree = parser
        .parse(src, None)?
        .context("No valid Markdown tree found")?;
    let root_node = NodeWithSource::new(src_tree.root_node(), src);

    let cursor = CursorWrapper::new(root_node.node.walk(), root_node.source);

    let mut patterns: Vec<MarkdownBody> = Vec::new();

    // Track the current language block for when we hit the actual content
    let mut current_code_block_language = None;

    // Track the current heading level, used for determining which patterns should be grouped together
    let mut current_heading = (1, "".to_string());

    let mut meta = GritPatternMetadata::default();

    for n in traverse(cursor, Order::Pre) {
        if n.node.kind() == "language" {
            current_code_block_language = Some(n.node.utf8_text(src.as_bytes()).unwrap());
        } else if n.node.kind() == "code_fence_content" {
            let content = n.node.utf8_text(src.as_bytes()).unwrap();
            let content = content.to_string();
            println!(
                "Processing code block (lang = {:?}): {:?}",
                current_code_block_language, content
            );
            if current_code_block_language == Some(std::borrow::Cow::Borrowed("grit")) {
                let definition = MarkdownBody {
                    body: content.to_string(),
                    position: n.node.range().start_point().into(),
                    section_heading: current_heading.1.clone(),
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
                if heading_level <= last_config.section_level + 1 {
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
        } else if n.node.kind() == "minus_metadata" {
            let metadata = n.node.utf8_text(src.as_bytes()).unwrap();
            let metadata = metadata.trim().to_string();
            let metadata = metadata.trim_start_matches('-').trim();
            let metadata = metadata.trim_end_matches('-').trim();
            meta = parse_metadata(metadata)?;
        } else if n.node.kind() == "paragraph" {
            let content = n.node.utf8_text(src.as_bytes()).unwrap();
            let content = content.trim().to_string();

            if meta.description.is_none() {
                meta.description = Some(content);
            }
        }
    }

    // Markdown patterns have a default level of info
    if meta.level.is_none() {
        meta.level = Some(EnforcementLevel::Info);
    }

    let mut grit_parser = make_grit_parser()?;

    let patterns = patterns
        .into_iter()
        .enumerate()
        .map(|(i, mut p)| {
            let local_name = if i == 0 {
                name.to_string()
            } else {
                format!("{}-{}", name, i)
            };
            let mut meta_copy = meta.clone();
            if meta_copy.title.is_none() {
                meta_copy.title = Some(p.section_heading);
            }
            if let Some(last_sample) = p.open_sample.take() {
                p.samples.push(last_sample);
            }
            let src_tree = grit_parser
                .parse(&p.body, None)?
                .ok_or_else(|| anyhow!("parse error"))?;
            if defines_itself(&NodeWithSource::new(src_tree.root_node(), &p.body), name)? {
                bail!("Pattern {} attempts to define itself - this is not allowed. Tip: Markdown patterns use the file name as their pattern name.", name);
            };
            Ok(ModuleGritPattern {
                config: GritDefinitionConfig {
                    name: local_name.clone(),
                    body: Some(p.body),
                    meta: meta_copy,
                    kind: Some(DefinitionKind::Pattern),
                    samples: Some(p.samples),
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
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(patterns)
}

pub fn get_body_from_md_content(content: &str) -> Result<String> {
    let patterns = get_patterns_from_md(
        &mut RichFile {
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
    let Some(range) = &sample.output_range else {
        bail!("Sample does not have an output range, cannot replace in file");
    };

    let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    if let Some(actual_output) = &sample.output {
        content.replace_range(
            range.start_byte as usize..range.end_byte as usize,
            actual_output,
        );
    } else {
        content.replace_range(range.start_byte as usize..range.end_byte as usize, "");
    };

    file.seek(SeekFrom::Start(0))?;
    file.write_all(content.as_bytes())?;
    file.set_len(content.len() as u64)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use insta::{assert_snapshot, assert_yaml_snapshot};

    use super::*;

    #[test]
    fn test_markdown_parser() {
        let module = Default::default();
        let mut rich_file = RichFile { path: "no_debugger.md".to_string(),content: r#"---
title: Remove `debugger` statement
tags: [fix]
---

The code in production should not contain a `debugger`. It causes the browser to stop executing the code and open the debugger.

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
        let patterns = get_patterns_from_md(&mut rich_file, &module, &None).unwrap();
        assert_eq!(patterns.len(), 1);
        println!("{:?}", patterns);
        assert_yaml_snapshot!(patterns);
    }

    #[test]
    fn test_sample_labels() {
        let module = Default::default();
        let mut rich_file = RichFile { path: "no_debugger.md".to_string(),content: r#"---
title: Remove `debugger` statement
tags: fix
---

The code in production should not contain a `debugger`. It causes the browser to stop executing the code and open the debugger.

```grit
engine marzano(0.1)
language js

debugger_statement() => .
```

## Remove debugger - pairs

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

## Remove debugger - group 2
Bad:
```javascript
function isTruthy(x) {
  debugger;
  return Boolean(x);
}
```

Good:
```typescript
function isTruthy(x) {
  return Boolean(x);
}
```

## Remove debugger - no match
Bad:
```javascript
function isTruthy(x) {
  debugger;
  return Boolean(x);
}
```
"#.to_string()};
        let patterns = get_patterns_from_md(&mut rich_file, &module, &None).unwrap();
        assert_eq!(patterns.len(), 1);
        println!("{:?}", patterns);
        assert_eq!(patterns[0].config.samples.as_ref().unwrap().len(), 3);
        assert_yaml_snapshot!(patterns);
    }

    #[test]
    fn test_multiple_markdown_patterns() {
        let module = Default::default();
        let mut rich_file = RichFile {
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
```

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
```

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
        let patterns = get_patterns_from_md(&mut rich_file, &module, &None).unwrap();
        assert_eq!(patterns.len(), 2);
        println!("{:?}", patterns);
        assert_yaml_snapshot!(patterns);
    }

    #[test]
    fn test_yaml_format() {
        let module = Default::default();
        let mut rich_file = RichFile {
            path: "concourse.md".to_string(),
            content: r#"
This is a markdown file with some yaml patterns

```grit
engine marzano(0.1)
language yaml

$_ => .
```

## Basic input

```yaml
foo:
  - name

  - bar
jobs:
  - across:
    - var: function
      values:
      - file1
      - file2
      - file3
    - var: x
      values:
      - one
      - two
    - var: y
      values:
      - a
      - b
    task: create-file
    params:
      FUNCTION: ((.:function))-js
    input_mapping:
      code: ((.:x))-js
    output_mapping:
      code: ((.:y))-js
```

```yaml
foo:
  - name

  - bar
jobs:
  - in_parallel:
      steps:
        - task: create-file-1
          params:
              FUNCTION: file1-js
          input_mapping:
              code: one-js
          output_mapping:
              code: a-js

        - task: create-file-2
          params:
              FUNCTION: file2-js
          input_mapping:
              code: one-js
          output_mapping:
              code: a-js

        - task: create-file-3
          params:
              FUNCTION: file3-js
          input_mapping:
              code: one-js
          output_mapping:
              code: a-js

        - task: create-file-4
          params:
              FUNCTION: file1-js
          input_mapping:
              code: two-js
          output_mapping:
              code: a-js

        - task: create-file-5
          params:
              FUNCTION: file2-js
          input_mapping:
              code: two-js
          output_mapping:
              code: a-js

        - task: create-file-6
          params:
              FUNCTION: file3-js
          input_mapping:
              code: two-js
          output_mapping:
              code: a-js

        - task: create-file-7
          params:
              FUNCTION: file1-js
          input_mapping:
              code: one-js
          output_mapping:
              code: b-js

        - task: create-file-8
          params:
              FUNCTION: file2-js
          input_mapping:
              code: one-js
          output_mapping:
              code: b-js

        - task: create-file-9
          params:
              FUNCTION: file3-js
          input_mapping:
              code: one-js
          output_mapping:
              code: b-js

        - task: create-file-10
          params:
              FUNCTION: file1-js
          input_mapping:
              code: two-js
          output_mapping:
              code: b-js

        - task: create-file-11
          params:
              FUNCTION: file2-js
          input_mapping:
              code: two-js
          output_mapping:
              code: b-js

        - task: create-file-12
          params:
              FUNCTION: file3-js
          input_mapping:
              code: two-js
          output_mapping:
              code: b-js

```

"#
            .to_string(),
        };
        let patterns = get_patterns_from_md(&mut rich_file, &module, &None).unwrap();
        assert_eq!(patterns.len(), 1);
        println!("{:?}", patterns);
        let sample_input = &patterns[0].config.samples.as_ref().unwrap()[0].input;
        let sample_output = &patterns[0].config.samples.as_ref().unwrap()[0]
            .output
            .as_ref()
            .unwrap();
        assert_snapshot!(sample_input);
        assert_snapshot!(sample_output);
    }

    #[test]
    fn test_frontmatter_enforcement() {
        let module = Default::default();
        let mut rich_file = RichFile {
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
        let patterns = get_patterns_from_md(&mut rich_file, &module, &None).unwrap();
        assert_eq!(patterns.len(), 1);
        println!("{:?}", patterns);
        assert_eq!(patterns[0].config.meta.level, Some(EnforcementLevel::Error));
    }

    #[test]
    fn with_non_matching_samples() {
        let module = Default::default();
        let mut rich_file = RichFile {
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
        let patterns = get_patterns_from_md(&mut rich_file, &module, &None).unwrap();
        assert_eq!(patterns.len(), 1);
        assert_yaml_snapshot!(patterns);
    }

    #[test]
    fn with_no_frontmatter() {
        let module = ModuleRepo::from_host_repo("github.com", "getgrit/rewriter").unwrap();
        let mut rich_file = RichFile {
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
        let patterns = get_patterns_from_md(&mut rich_file, &Some(module), &None).unwrap();
        assert_eq!(patterns.len(), 1);
        assert_yaml_snapshot!(patterns);
    }

    #[test]
    fn test_weird_patterns() {
        let module = ModuleRepo::from_host_repo("github.com", "getgrit/rewriter").unwrap();
        let mut rich_file = RichFile {
            path: "CloneActivities.md".to_string(),
            content: r#"---
title: Oracle to PG: Dollar quote stored procedure body
---

In Postgres, function and procedure bodies need to be wrapped in $$dollar quotes$$.
This pattern wraps a PLSQL `CREATE PROCEDURE` body in dollar quotes and adds a language specifier.

```grit
engine marzano(0.1)
language sql

pattern dollar_quote_procedure_body() {
`CREATE PROCEDURE $name($args) AS $decl $block;` as $proc where {
    $block => `$$$block;\n$$ LANGUAGE plpgsql`,
    $decl => `DECLARE\n $decl`
    }
}
dollar_quote_procedure_body()
```

## Basic procedure

```sql
CREATE PROCEDURE remove_emp (employee_id int) AS
    tot_emps int;
    BEGIN
        DELETE FROM employees
        WHERE employees.employee_id = remove_emp.employee_id;
    tot_emps := tot_emps - 1;
    END;
```

```sql
CREATE PROCEDURE remove_emp (employee_id int) AS
    DECLARE
    tot_emps int;
    $$BEGIN
        DELETE FROM employees
        WHERE employees.employee_id = remove_emp.employee_id;
    tot_emps := tot_emps - 1;
    END;
$$ LANGUAGE plpgsql;
```
"#
            .to_string(),
        };
        let patterns = get_patterns_from_md(&mut rich_file, &Some(module), &None).unwrap();
        assert_eq!(patterns.len(), 1);
        assert_yaml_snapshot!(patterns);

        // Check that the output range is correct
        let sample = &patterns[0].config.samples.as_ref().unwrap()[0];
        let output_content = rich_file.content[sample.output_range.as_ref().unwrap().start_byte
            as usize
            ..sample.output_range.as_ref().unwrap().end_byte as usize]
            .to_string();
        assert_eq!(&output_content, sample.output.as_ref().unwrap());
        assert_snapshot!(output_content);
    }
}
