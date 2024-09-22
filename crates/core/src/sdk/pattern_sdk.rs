pub enum PatternBuilder {
    Contains { child: Box<PatternBuilder> },
}
