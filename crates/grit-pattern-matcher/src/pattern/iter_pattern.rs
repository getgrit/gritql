use super::{
    accessor::Accessor, container::Container, dynamic_snippet::DynamicPattern,
    list_index::ListIndex, patterns::Pattern, predicates::Predicate, regex::RegexLike,
};
use crate::{
    context::{QueryContext, StaticDefinitions},
    pattern::{
        ast_node_pattern::AstNodePattern,
        list_index::{ContainerOrIndex, ListOrContainer},
        patterns::CodeSnippet,
    },
};

pub struct PatternOrPredicateIterator<'a, Q: QueryContext> {
    patterns: Vec<PatternOrPredicate<'a, Q>>,
    definitions: &'a StaticDefinitions<'a, Q>,
}

impl<'a, Q: QueryContext> Iterator for PatternOrPredicateIterator<'a, Q> {
    type Item = PatternOrPredicate<'a, Q>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pattern) = self.patterns.pop() {
            self.patterns.extend(pattern.children(self.definitions));
            Some(pattern)
        } else {
            None
        }
    }
}

impl<'a, Q: QueryContext> PatternOrPredicateIterator<'a, Q> {
    fn from_pattern(pattern: &'a Pattern<Q>, definitions: &'a StaticDefinitions<Q>) -> Self {
        Self {
            patterns: vec![PatternOrPredicate::Pattern(pattern)],
            definitions,
        }
    }
    fn from_predicate(predicate: &'a Predicate<Q>, definitions: &'a StaticDefinitions<Q>) -> Self {
        Self {
            patterns: vec![PatternOrPredicate::Predicate(predicate)],
            definitions,
        }
    }
}

// todo maybe add variable?
#[derive(Clone, Copy)]
pub enum PatternOrPredicate<'a, Q: QueryContext> {
    Pattern(&'a Pattern<Q>),
    Predicate(&'a Predicate<Q>),
    DynamicPattern(&'a DynamicPattern<Q>),
}

impl<'a, Q: QueryContext> PatternOrPredicate<'a, Q> {
    fn children(&self, definitions: &'a StaticDefinitions<Q>) -> Vec<PatternOrPredicate<'a, Q>> {
        match self {
            PatternOrPredicate::Pattern(p) => p.children(definitions),
            PatternOrPredicate::Predicate(p) => p.children(definitions),
            PatternOrPredicate::DynamicPattern(p) => p.children(definitions),
        }
    }
}

impl<Q: QueryContext> Predicate<Q> {
    pub fn iter<'a>(
        &'a self,
        definitions: &'a StaticDefinitions<Q>,
    ) -> PatternOrPredicateIterator<'a, Q> {
        PatternOrPredicateIterator::from_predicate(self, definitions)
    }

    fn children<'a>(
        &'a self,
        definitions: &'a StaticDefinitions<Q>,
    ) -> Vec<PatternOrPredicate<'a, Q>> {
        match self {
            Predicate::Call(call) => {
                let mut base = args_children(&call.args, definitions);
                let def = definitions.get_predicate(call.index);
                if let Some(def) = def {
                    base.push(PatternOrPredicate::Predicate(&def.predicate));
                }
                base
            }
            Predicate::Not(not) => vec![PatternOrPredicate::Predicate(&not.predicate)],
            Predicate::If(if_) => vec![
                PatternOrPredicate::Predicate(&if_.if_),
                PatternOrPredicate::Predicate(&if_.then),
                PatternOrPredicate::Predicate(&if_.else_),
            ],
            Predicate::True => vec![],
            Predicate::False => vec![],
            Predicate::Or(or) => predicates_children(&or.predicates, definitions),
            Predicate::Maybe(m) => vec![PatternOrPredicate::Predicate(&m.predicate)],
            Predicate::And(and) => predicates_children(&and.predicates, definitions),
            Predicate::Any(any) => predicates_children(&any.predicates, definitions),
            Predicate::Rewrite(rewrite) => {
                let mut res = rewrite.right.children(definitions);
                res.push(PatternOrPredicate::Pattern(&rewrite.left));
                res
            }
            Predicate::Log(log) => log
                .message
                .iter()
                .map(PatternOrPredicate::Pattern)
                .collect(),
            Predicate::Match(match_) => match_
                .pattern
                .iter()
                .map(PatternOrPredicate::Pattern)
                .collect(),
            Predicate::Equal(equal) => vec![PatternOrPredicate::Pattern(&equal.pattern)],
            Predicate::Assignment(assignment) => {
                vec![PatternOrPredicate::Pattern(&assignment.pattern)]
            }
            Predicate::Accumulate(accumulate) => vec![
                PatternOrPredicate::Pattern(&accumulate.left),
                PatternOrPredicate::Pattern(&accumulate.right),
            ],
            Predicate::Return(return_) => vec![PatternOrPredicate::Pattern(&return_.pattern)],
        }
    }
}

impl<Q: QueryContext> Container<Q> {
    fn children<'a>(
        &'a self,
        definitions: &'a StaticDefinitions<Q>,
    ) -> Vec<PatternOrPredicate<'a, Q>> {
        match self {
            Container::Variable(_) => vec![],
            Container::Accessor(a) => a.children(definitions),
            Container::ListIndex(l) => l.children(definitions),
            Container::FunctionCall(f) => {
                let mut base = args_children(&f.args, definitions);
                let def = definitions.get_function(f.index);
                if let Some(def) = def {
                    base.push(PatternOrPredicate::Predicate(&def.function));
                }
                base
            }
        }
    }
}

impl<Q: QueryContext> Accessor<Q> {
    fn children<'a>(
        &'a self,
        definitions: &'a StaticDefinitions<Q>,
    ) -> Vec<PatternOrPredicate<'a, Q>> {
        match &self.map {
            super::accessor::AccessorMap::Container(c) => c.children(definitions),
            super::accessor::AccessorMap::Map(m) => m
                .elements
                .values()
                .map(PatternOrPredicate::Pattern)
                .collect(),
        }
    }
}

impl<Q: QueryContext> DynamicPattern<Q> {
    fn children<'a>(
        &'a self,
        definitions: &'a StaticDefinitions<Q>,
    ) -> Vec<PatternOrPredicate<'a, Q>> {
        match &self {
            super::dynamic_snippet::DynamicPattern::Variable(_) => Vec::new(),
            super::dynamic_snippet::DynamicPattern::Accessor(a) => a.children(definitions),
            super::dynamic_snippet::DynamicPattern::ListIndex(l) => l.children(definitions),
            super::dynamic_snippet::DynamicPattern::Snippet(_) => Vec::new(),
            super::dynamic_snippet::DynamicPattern::List(l) => l
                .elements
                .iter()
                .flat_map(|d| d.children(definitions))
                .collect(),
            super::dynamic_snippet::DynamicPattern::CallBuiltIn(c) => {
                args_children(&c.args, definitions)
            }
            super::dynamic_snippet::DynamicPattern::CallFunction(c) => {
                args_children(&c.args, definitions)
            }
            super::dynamic_snippet::DynamicPattern::CallForeignFunction(c) => {
                args_children(&c.args, definitions)
            }
        }
    }
}

impl<Q: QueryContext> ListIndex<Q> {
    fn children<'a>(
        &'a self,
        definitions: &'a StaticDefinitions<Q>,
    ) -> Vec<PatternOrPredicate<'a, Q>> {
        let mut v = Vec::new();
        let list = match &self.list {
            ListOrContainer::Container(c) => c.children(definitions),
            ListOrContainer::List(l) => patterns_children(&l.patterns, definitions),
        };
        let index = match &self.index {
            ContainerOrIndex::Container(c) => c.children(definitions),
            ContainerOrIndex::Index(_) => Vec::new(),
        };
        v.extend(list);
        v.extend(index);
        v
    }
}

fn args_children<'a, Q: QueryContext>(
    args: &'a [Option<Pattern<Q>>],
    _definitions: &'a StaticDefinitions<Q>,
) -> Vec<PatternOrPredicate<'a, Q>> {
    args.iter()
        .flat_map(|p| p.as_ref().map(PatternOrPredicate::Pattern))
        .collect()
}

fn patterns_children<'a, Q: QueryContext>(
    patterns: &'a [Pattern<Q>],
    _definitions: &'a StaticDefinitions<Q>,
) -> Vec<PatternOrPredicate<'a, Q>> {
    patterns.iter().map(PatternOrPredicate::Pattern).collect()
}

fn predicates_children<'a, Q: QueryContext>(
    predicates: &'a [Predicate<Q>],
    _definitions: &'a StaticDefinitions<Q>,
) -> Vec<PatternOrPredicate<'a, Q>> {
    predicates
        .iter()
        .map(PatternOrPredicate::Predicate)
        .collect()
}

impl<Q: QueryContext> Pattern<Q> {
    pub fn iter<'a>(
        &'a self,
        definitions: &'a StaticDefinitions<Q>,
    ) -> PatternOrPredicateIterator<'a, Q> {
        PatternOrPredicateIterator::from_pattern(self, definitions)
    }

    fn children<'a>(
        &'a self,
        definitions: &'a StaticDefinitions<Q>,
    ) -> Vec<PatternOrPredicate<'a, Q>> {
        match self {
            Pattern::AstNode(a) => a.children(definitions),
            Pattern::List(l) => patterns_children(&l.patterns, definitions),
            Pattern::ListIndex(l) => l.children(definitions),
            Pattern::Map(m) => m
                .elements
                .values()
                .map(PatternOrPredicate::Pattern)
                .collect(),
            Pattern::Accessor(a) => a.children(definitions),
            Pattern::Call(c) => {
                let mut base = args_children(&c.args, definitions);
                let def = definitions.get_pattern(c.index);
                if let Some(def) = def {
                    base.push(PatternOrPredicate::Pattern(def.pattern()));
                }
                base
            }
            Pattern::Regex(r) => {
                if let RegexLike::Pattern(p) = &r.regex {
                    p.children(definitions)
                } else {
                    Vec::new()
                }
            }
            Pattern::File(f) => {
                let mut v = Vec::new();
                let n = f.name.children(definitions);
                let b = f.body.children(definitions);
                v.extend(n);
                v.extend(b);
                v
            }
            Pattern::Files(f) => f.pattern.children(definitions),
            Pattern::Bubble(b) => {
                let mut children = args_children(&b.args, definitions);
                children.extend(b.pattern_def.pattern().children(definitions));
                children
            }
            Pattern::Limit(l) => l.pattern.children(definitions),
            Pattern::CallBuiltIn(c) => args_children(&c.args, definitions),
            Pattern::CallFunction(c) => {
                let mut children = args_children(&c.args, definitions);
                let def = definitions.get_function(c.index);
                if let Some(def) = def {
                    children.extend(def.function.children(definitions));
                }
                children
            }
            Pattern::CallForeignFunction(c) => args_children(&c.args, definitions),
            Pattern::CallbackPattern(_) => vec![],
            Pattern::Assignment(a) => vec![PatternOrPredicate::Pattern(&a.pattern)],
            Pattern::Accumulate(a) => vec![
                PatternOrPredicate::Pattern(&a.left),
                PatternOrPredicate::Pattern(&a.right),
            ],
            Pattern::And(a) => patterns_children(&a.patterns, definitions),
            Pattern::Or(o) => patterns_children(&o.patterns, definitions),
            Pattern::Maybe(m) => vec![PatternOrPredicate::Pattern(&m.pattern)],
            Pattern::Any(a) => patterns_children(&a.patterns, definitions),
            Pattern::Not(n) => vec![PatternOrPredicate::Pattern(&n.pattern)],
            Pattern::If(i) => vec![
                PatternOrPredicate::Predicate(&i.if_),
                PatternOrPredicate::Pattern(&i.then),
                PatternOrPredicate::Pattern(&i.else_),
            ],
            Pattern::Undefined => Vec::new(),
            Pattern::Top => Vec::new(),
            Pattern::Bottom => Vec::new(),
            Pattern::Underscore => Vec::new(),
            Pattern::StringConstant(_) => Vec::new(),
            Pattern::AstLeafNode(_) => Vec::new(),
            Pattern::IntConstant(_) => Vec::new(),
            Pattern::FloatConstant(_) => Vec::new(),
            Pattern::BooleanConstant(_) => Vec::new(),
            Pattern::Dynamic(d) => d.children(definitions),
            Pattern::CodeSnippet(c) => {
                let mut v = Vec::new();
                let p = c.patterns().map(|p| PatternOrPredicate::Pattern(p));
                let d = c
                    .dynamic_snippet()
                    .map(|d| d.children(definitions))
                    .unwrap_or(Vec::new());
                v.extend(p);
                v.extend(d);
                v
            }
            Pattern::Variable(_) => Vec::new(),
            Pattern::Rewrite(r) => {
                vec![
                    PatternOrPredicate::Pattern(&r.left),
                    PatternOrPredicate::DynamicPattern(&r.right),
                ]
            }
            Pattern::Log(l) => l.message.iter().map(PatternOrPredicate::Pattern).collect(),
            Pattern::Range(_) => Vec::new(),
            Pattern::Contains(c) => c
                .until
                .iter()
                .map(PatternOrPredicate::Pattern)
                .chain(Some(PatternOrPredicate::Pattern(&c.contains)))
                .collect(),
            Pattern::Includes(i) => vec![PatternOrPredicate::Pattern(&i.includes)],
            Pattern::Within(w) => vec![PatternOrPredicate::Pattern(&w.pattern)],
            Pattern::After(a) => vec![PatternOrPredicate::Pattern(&a.after)],
            Pattern::Before(b) => vec![PatternOrPredicate::Pattern(&b.before)],
            Pattern::Where(w) => vec![
                PatternOrPredicate::Pattern(&w.pattern),
                PatternOrPredicate::Predicate(&w.side_condition),
            ],
            Pattern::Some(s) => vec![PatternOrPredicate::Pattern(&s.pattern)],
            Pattern::Every(a) => vec![PatternOrPredicate::Pattern(&a.pattern)],
            Pattern::Add(a) => vec![
                PatternOrPredicate::Pattern(&a.lhs),
                PatternOrPredicate::Pattern(&a.rhs),
            ],
            Pattern::Subtract(a) => vec![
                PatternOrPredicate::Pattern(&a.lhs),
                PatternOrPredicate::Pattern(&a.rhs),
            ],
            Pattern::Multiply(a) => vec![
                PatternOrPredicate::Pattern(&a.lhs),
                PatternOrPredicate::Pattern(&a.rhs),
            ],
            Pattern::Divide(a) => vec![
                PatternOrPredicate::Pattern(&a.lhs),
                PatternOrPredicate::Pattern(&a.rhs),
            ],
            Pattern::Modulo(a) => vec![
                PatternOrPredicate::Pattern(&a.lhs),
                PatternOrPredicate::Pattern(&a.rhs),
            ],
            Pattern::Dots => Vec::new(),
            Pattern::Sequential(s) => s
                .iter()
                .map(|s| PatternOrPredicate::Pattern(&s.pattern))
                .collect(),
            Pattern::Like(l) => vec![
                PatternOrPredicate::Pattern(&l.like),
                PatternOrPredicate::Pattern(&l.threshold),
            ],
        }
    }
}
