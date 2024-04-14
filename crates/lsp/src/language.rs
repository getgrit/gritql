use marzano_language::target_language::{PatternLanguage, TargetLanguage};

pub fn language_id_to_pattern_language(language_id: &str) -> Option<PatternLanguage> {
    match language_id {
        "javascript" | "typescript" | "javascriptreact" | "typescriptreact" => {
            Some(PatternLanguage::Tsx)
        }
        "html" => Some(PatternLanguage::Html),
        "css" => Some(PatternLanguage::Css),
        "json" | "jsonc" => Some(PatternLanguage::Json),
        "java" => Some(PatternLanguage::Java),
        "csharp" => Some(PatternLanguage::CSharp),
        "python" => Some(PatternLanguage::Python),
        "markdown" => Some(PatternLanguage::MarkdownInline),
        "go" => Some(PatternLanguage::Go),
        "rust" => Some(PatternLanguage::Rust),
        "ruby" => Some(PatternLanguage::Ruby),
        "solidity" => Some(PatternLanguage::Solidity),
        "hcl" | "terraform" => Some(PatternLanguage::Hcl),
        "yaml" => Some(PatternLanguage::Yaml),
        "sql" => Some(PatternLanguage::Sql),
        "vue" => Some(PatternLanguage::Vue),
        "toml" => Some(PatternLanguage::Toml),
        "php" => Some(PatternLanguage::PhpOnly),
        _ => None,
    }
}

#[allow(dead_code)]
pub fn target_language_to_language_id(target_language: TargetLanguage) -> &'static str {
    match target_language {
        TargetLanguage::JavaScript(_) => "javascript",
        TargetLanguage::TypeScript(_) => "typescript",
        TargetLanguage::Html(_) => "html",
        TargetLanguage::Css(_) => "css",
        TargetLanguage::Json(_) => "json",
        TargetLanguage::Java(_) => "java",
        TargetLanguage::CSharp(_) => "csharp",
        TargetLanguage::Python(_) => "python",
        TargetLanguage::MarkdownInline(_) => "markdown",
        TargetLanguage::Go(_) => "go",
        TargetLanguage::Rust(_) => "rust",
        TargetLanguage::Ruby(_) => "ruby",
        TargetLanguage::Solidity(_) => "solidity",
        TargetLanguage::Hcl(_) => "hcl",
        TargetLanguage::Yaml(_) => "yaml",
        TargetLanguage::Tsx(_) => "typescriptreact",
        TargetLanguage::MarkdownBlock(_) => "markdown",
        TargetLanguage::Sql(_) => "sql",
        TargetLanguage::Vue(_) => "vue",
        TargetLanguage::Toml(_) => "toml",
        TargetLanguage::Php(_) => "php",
        TargetLanguage::PhpOnly(_) => "php",
    }
}

#[allow(dead_code)]
pub fn extension_to_language_id(extension: &str) -> Option<String> {
    let language = TargetLanguage::from_extension(extension)?;
    Some(target_language_to_language_id(language).to_string())
}

