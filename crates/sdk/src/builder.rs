/// PatternBuilder provides a high-level interface for building patterns
struct PatternBuilder {
    language: TargetLanguage,
}

impl PatternBuilder {
    pub fn new(language: Language) -> Self {
        Self { language }
    }
}
