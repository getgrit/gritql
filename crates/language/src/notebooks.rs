use grit_util::Ast;
use grit_util::AstCursor;
use grit_util::AstNode;
use grit_util::ByteRange;
use grit_util::FileOrigin;

use std::path::Path;

use grit_util::traverse;
use grit_util::Order;

use grit_util::{AnalysisLogs, SnippetTree};
use marzano_util::cursor_wrapper::CursorWrapper;

use crate::sourcemap::EmbeddedSourceMap;
use crate::sourcemap::SourceMapSection;
use crate::sourcemap::SourceValueFormat;
use crate::{
    json::Json,
    language::{MarzanoLanguage, MarzanoParser, Tree},
};

const SUPPORTED_VERSION: i64 = 4;

/// Returns `true` if a cell should be ignored due to the use of cell magics.
/// Borrowed from [ruff](https://github.com/astral-sh/ruff/blob/33fd50027cb24e407746da339bdf2461df194d96/crates/ruff_notebook/src/cell.rs)
fn is_magic_cell<'a>(lines: impl Iterator<Item = &'a str>) -> bool {
    let mut lines = lines.peekable();

    // Detect automatic line magics (automagic), which aren't supported by the parser. If a line
    // magic uses automagic, Jupyter doesn't allow following it with non-magic lines anyway, so
    // we aren't missing out on any valid Python code.
    //
    // For example, this is valid:
    // ```jupyter
    // cat /path/to/file
    // cat /path/to/file
    // ```
    //
    // But this is invalid:
    // ```jupyter
    // cat /path/to/file
    // x = 1
    // ```
    //
    // See: https://ipython.readthedocs.io/en/stable/interactive/magics.html
    if let Some(line) = lines.peek() {
        let mut tokens = line.split_whitespace();

        // The first token must be an automagic, like `load_exit`.
        if tokens.next().is_some_and(|token| {
            matches!(
                token,
                "alias"
                    | "alias_magic"
                    | "autoawait"
                    | "autocall"
                    | "automagic"
                    | "bookmark"
                    | "cd"
                    | "code_wrap"
                    | "colors"
                    | "conda"
                    | "config"
                    | "debug"
                    | "dhist"
                    | "dirs"
                    | "doctest_mode"
                    | "edit"
                    | "env"
                    | "gui"
                    | "history"
                    | "killbgscripts"
                    | "load"
                    | "load_ext"
                    | "loadpy"
                    | "logoff"
                    | "logon"
                    | "logstart"
                    | "logstate"
                    | "logstop"
                    | "lsmagic"
                    | "macro"
                    | "magic"
                    | "mamba"
                    | "matplotlib"
                    | "micromamba"
                    | "notebook"
                    | "page"
                    | "pastebin"
                    | "pdb"
                    | "pdef"
                    | "pdoc"
                    | "pfile"
                    | "pinfo"
                    | "pinfo2"
                    | "pip"
                    | "popd"
                    | "pprint"
                    | "precision"
                    | "prun"
                    | "psearch"
                    | "psource"
                    | "pushd"
                    | "pwd"
                    | "pycat"
                    | "pylab"
                    | "quickref"
                    | "recall"
                    | "rehashx"
                    | "reload_ext"
                    | "rerun"
                    | "reset"
                    | "reset_selective"
                    | "run"
                    | "save"
                    | "sc"
                    | "set_env"
                    | "sx"
                    | "system"
                    | "tb"
                    | "time"
                    | "timeit"
                    | "unalias"
                    | "unload_ext"
                    | "who"
                    | "who_ls"
                    | "whos"
                    | "xdel"
                    | "xmode"
            )
        }) {
            // The second token must _not_ be an operator, like `=` (to avoid false positives).
            // The assignment operators can never follow an automagic. Some binary operators
            // _can_, though (e.g., `cd -` is valid), so we omit them.
            if !tokens.next().is_some_and(|token| {
                matches!(
                    token,
                    "=" | "+=" | "-=" | "*=" | "/=" | "//=" | "%=" | "**=" | "&=" | "|=" | "^="
                )
            }) {
                return true;
            }
        }
    }

    // Detect cell magics (which operate on multiple lines).
    lines.any(|line| {
        let Some(first) = line.split_whitespace().next() else {
            return false;
        };
        if first.len() < 2 {
            return false;
        }
        let Some(command) = first.strip_prefix("%%") else {
            return false;
        };
        // These cell magics are special in that the lines following them are valid
        // Python code and the variables defined in that scope are available to the
        // rest of the notebook.
        //
        // For example:
        //
        // Cell 1:
        // ```python
        // x = 1
        // ```
        //
        // Cell 2:
        // ```python
        // %%time
        // y = x
        // ```
        //
        // Cell 3:
        // ```python
        // print(y)  # Here, `y` is available.
        // ```
        //
        // This is to avoid false positives when these variables are referenced
        // elsewhere in the notebook.
        !matches!(
            command,
            "capture" | "debug" | "prun" | "pypy" | "python" | "python3" | "time" | "timeit"
        )
    })
}

/// Custom Python parser, to include notebooks
pub(crate) struct MarzanoNotebookParser(MarzanoParser);

impl MarzanoNotebookParser {
    pub(crate) fn new<'a>(lang: &impl MarzanoLanguage<'a>) -> Self {
        Self(MarzanoParser::new(lang))
    }

    fn parse_file_as_notebook(
        &mut self,
        body: &str,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
    ) -> Option<Tree> {
        let mut inner_code_body = String::new();
        let mut source_map = EmbeddedSourceMap::new(body);

        let mut nbformat_version: Option<i64> = None;

        let json = Json::new(None);
        let mut parser = json.get_parser();
        let tree = parser.parse_file(body, None, logs, FileOrigin::Fresh)?;
        let root = tree.root_node().node;
        let cursor = root.walk();

        for n in traverse(CursorWrapper::new(cursor, body), Order::Pre) {
            if n.node.kind() == "pair"
                && n.child_by_field_name("key")
                    .and_then(|key| key.node.utf8_text(body.as_bytes()).ok())
                    .map(|key| key == "\"nbformat\"")
                    .unwrap_or(false)
            {
                let value = n
                    .child_by_field_name("value")
                    .and_then(|value| value.node.utf8_text(body.as_bytes()).ok())
                    .map(|value| value.parse::<i64>().unwrap())
                    .unwrap_or(0);
                if value != SUPPORTED_VERSION {
                    logs.add_warning(
                        path.map(|m| m.into()),
                        format!("Unsupported version {} found", value),
                    );
                    return None;
                }
                nbformat_version = Some(value);
            }
            if n.node.kind() != "object" {
                continue;
            }

            let mut cursor = n.walk();

            let mut is_code_cell = false;

            let mut source_ranges: Option<(String, SourceMapSection)> = None;

            cursor.goto_first_child(); // Enter the object
            while cursor.goto_next_sibling() {
                // Iterate over the children of the object
                let node = cursor.node();

                if node.node.kind() == "pair"
                    && node
                        .child_by_field_name("key")
                        .and_then(|key| key.node.utf8_text(body.as_bytes()).ok())
                        .map(|key| key == "\"cell_type\"")
                        .unwrap_or(false)
                    && node
                        .child_by_field_name("value")
                        .and_then(|value| value.node.utf8_text(body.as_bytes()).ok())
                        .map(|value| value == "\"code\"")
                        .unwrap_or(false)
                {
                    is_code_cell = true;
                }

                if node.node.kind() == "pair"
                    && node
                        .child_by_field_name("key")
                        .and_then(|key| key.node.utf8_text(body.as_bytes()).ok())
                        .map(|key| key == "\"source\"")
                        .unwrap_or(false)
                {
                    if let Some(value) = node.child_by_field_name("value") {
                        let range = value.node.range();
                        let text = value.node.utf8_text(body.as_bytes()).ok()?;
                        let value: serde_json::Value = serde_json::from_str(&text).ok()?;

                        let (mut this_content, format) = match value {
                            serde_json::Value::Array(value) => (
                                value
                                    .iter()
                                    .map(|v| v.as_str().unwrap_or(""))
                                    .collect::<Vec<&str>>()
                                    .join(""),
                                SourceValueFormat::Array,
                            ),
                            serde_json::Value::String(s) => (s, SourceValueFormat::String),
                            _ => {
                                logs.add_warning(
                                    path.map(|m| m.into()),
                                    "Unsupported cell source format, expected a string or array of strings".to_string(),
                                );
                                continue;
                            }
                        };
                        // Add a newline to separate cells
                        this_content.push('\n');
                        let inner_range_end = inner_code_body.len() + this_content.len();
                        source_ranges = Some((
                            this_content,
                            SourceMapSection {
                                outer_range: ByteRange::new(
                                    range.start_byte().try_into().unwrap(),
                                    range.end_byte().try_into().unwrap(),
                                ),
                                inner_range_end,
                                format,
                                inner_end_trim: 1,
                            },
                        ));
                    }
                }
            }

            if is_code_cell {
                if let Some(source_range) = source_ranges {
                    let (content, section) = source_range;
                    if !is_magic_cell(content.lines()) {
                        println!("Adding section: {:?}", content);
                        inner_code_body.push_str(&content);
                        source_map.add_section(section);
                    } else {
                        println!("Magic cell found, skipping: {}", content);
                    }
                }
            }

            cursor.goto_parent(); // Exit the object
        }

        // Confirm we have a version
        if nbformat_version.is_none() {
            logs.add_warning(
                path.map(|m| m.into()),
                "No nbformat version found".to_string(),
            );
            return None;
        }

        self.0
            .parser
            .parse(inner_code_body.clone(), None)
            .ok()?
            .map(|tree| {
                let mut tree = Tree::new(tree, inner_code_body);
                tree.source_map = Some(source_map);
                tree
            })
    }
}

impl grit_util::Parser for MarzanoNotebookParser {
    type Tree = Tree;

    fn parse_file(
        &mut self,
        body: &str,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
        old_tree: FileOrigin<'_, Tree>,
    ) -> Option<Tree> {
        if path
            .and_then(Path::extension)
            .is_some_and(|ext| ext == "ipynb")
            && old_tree.is_fresh()
        {
            let tree = self.parse_file_as_notebook(body, path, logs);
            if let Some(tree) = tree {
                return Some(tree);
            }
        }

        self.0.parse_file(body, path, logs, old_tree)
    }

    fn parse_snippet(
        &mut self,
        pre: &'static str,
        source: &str,
        post: &'static str,
    ) -> SnippetTree<Tree> {
        self.0.parse_snippet(pre, source, post)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::python::Python;
    use grit_util::Parser as _;

    #[test]
    fn simple_notebook() {
        let code = include_str!("../../../crates/cli_bin/fixtures/notebooks/tiny_nb.ipynb");
        let mut parser = MarzanoNotebookParser::new(&Python::new(None));
        let tree = parser
            .parse_file(code, None, &mut AnalysisLogs::default(), FileOrigin::Fresh)
            .unwrap();

        let cursor = tree.root_node().node.walk();

        for n in traverse(CursorWrapper::new(cursor, code), Order::Pre) {
            println!("Node kind: {}", n.node.kind());
            assert!(
                !n.node.is_error(),
                "Node is an error: {}",
                n.node.utf8_text(code.as_bytes()).unwrap()
            );
        }
    }
}
