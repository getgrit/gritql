use crate::pattern::list_index::{ContainerOrIndex, ListOrContainer};

use super::{
    accessor::Accessor, container::Container, dynamic_snippet::DynamicPattern,
    list_index::ListIndex, patterns::Pattern, predicates::Predicate, regex::RegexLike,
};

pub(crate) struct PatternOrPredicateIterator<'a> {
    patterns: Vec<PatternOrPredicate<'a>>,
}

impl<'a> Iterator for PatternOrPredicateIterator<'a> {
    type Item = PatternOrPredicate<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pattern) = self.patterns.pop() {
            self.patterns.extend(&pattern.children());
            Some(pattern)
        } else {
            None
        }
    }
}

impl<'a> PatternOrPredicateIterator<'a> {
    fn from_pattern(pattern: &'a Pattern) -> Self {
        Self {
            patterns: vec![PatternOrPredicate::Pattern(pattern)],
        }
    }
    fn from_predicate(predicate: &'a Predicate) -> Self {
        Self {
            patterns: vec![PatternOrPredicate::Predicate(predicate)],
        }
    }
}

// todo maybe add variable?
#[derive(Clone, Copy)]
pub(crate) enum PatternOrPredicate<'a> {
    Pattern(&'a Pattern),
    Predicate(&'a Predicate),
}

impl<'a> PatternOrPredicate<'a> {
    fn children(&self) -> Vec<PatternOrPredicate<'a>> {
        match self {
            PatternOrPredicate::Pattern(p) => p.children(),
            PatternOrPredicate::Predicate(p) => p.children(),
        }
    }
}

impl Predicate {
    pub(crate) fn iter(&self) -> PatternOrPredicateIterator {
        PatternOrPredicateIterator::from_predicate(self)
    }

    fn children(&self) -> Vec<PatternOrPredicate> {
        match self {
            Predicate::Call(call) => args_children(&call.args),
            Predicate::Not(not) => vec![PatternOrPredicate::Predicate(&not.predicate)],
            Predicate::If(if_) => vec![
                PatternOrPredicate::Predicate(&if_.if_),
                PatternOrPredicate::Predicate(&if_.then),
                PatternOrPredicate::Predicate(&if_.else_),
            ],
            Predicate::True => vec![],
            Predicate::False => vec![],
            Predicate::Or(or) => predicates_children(&or.predicates),
            Predicate::Maybe(m) => vec![PatternOrPredicate::Predicate(&m.predicate)],
            Predicate::And(and) => predicates_children(&and.predicates),
            Predicate::Any(any) => predicates_children(&any.predicates),
            Predicate::Rewrite(rewrite) => {
                let mut res = rewrite.right.children();
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

impl Container {
    fn children(&self) -> Vec<PatternOrPredicate> {
        match self {
            Container::Variable(_) => vec![],
            Container::Accessor(a) => a.children(),
            Container::ListIndex(l) => l.children(),
        }
    }
}

impl Accessor {
    fn children(&self) -> Vec<PatternOrPredicate> {
        match &self.map {
            super::accessor::AccessorMap::Container(c) => c.children(),
            super::accessor::AccessorMap::Map(m) => m
                .elements
                .values()
                .map(PatternOrPredicate::Pattern)
                .collect(),
        }
    }
}

impl DynamicPattern {
    fn children(&self) -> Vec<PatternOrPredicate> {
        match &self {
            super::dynamic_snippet::DynamicPattern::Variable(_) => Vec::new(),
            super::dynamic_snippet::DynamicPattern::Accessor(a) => a.children(),
            super::dynamic_snippet::DynamicPattern::ListIndex(l) => l.children(),
            super::dynamic_snippet::DynamicPattern::Snippet(_) => Vec::new(),
            super::dynamic_snippet::DynamicPattern::List(l) => {
                l.elements.iter().flat_map(|d| d.children()).collect()
            }
            super::dynamic_snippet::DynamicPattern::CallBuiltIn(c) => args_children(&c.args),
            super::dynamic_snippet::DynamicPattern::CallFunction(c) => args_children(&c.args),
            super::dynamic_snippet::DynamicPattern::CallForeignFunction(c) => {
                args_children(&c.args)
            }
        }
    }
}

impl ListIndex {
    fn children(&self) -> Vec<PatternOrPredicate> {
        let mut v = Vec::new();
        let list = match &self.list {
            ListOrContainer::Container(c) => c.children(),
            ListOrContainer::List(l) => patterns_children(&l.patterns),
        };
        let index = match &self.index {
            ContainerOrIndex::Container(c) => c.children(),
            ContainerOrIndex::Index(_) => Vec::new(),
        };
        v.extend(list);
        v.extend(index);
        v
    }
}

fn args_children(args: &[Option<Pattern>]) -> Vec<PatternOrPredicate> {
    args.iter()
        .flat_map(|p| p.as_ref().map(PatternOrPredicate::Pattern))
        .collect()
}

fn patterns_children(patterns: &[Pattern]) -> Vec<PatternOrPredicate> {
    patterns.iter().map(PatternOrPredicate::Pattern).collect()
}

fn predicates_children(predicates: &[Predicate]) -> Vec<PatternOrPredicate> {
    predicates
        .iter()
        .map(PatternOrPredicate::Predicate)
        .collect()
}

impl Pattern {
    pub(crate) fn iter(&self) -> PatternOrPredicateIterator {
        PatternOrPredicateIterator::from_pattern(self)
    }

    fn children(&self) -> Vec<PatternOrPredicate> {
        match self {
            Pattern::ASTNode(a) => a
                .args
                .iter()
                .map(|a| PatternOrPredicate::Pattern(&a.2))
                .collect(),
            Pattern::List(l) => patterns_children(&l.patterns),
            Pattern::ListIndex(l) => l.children(),
            Pattern::Map(m) => m
                .elements
                .values()
                .map(PatternOrPredicate::Pattern)
                .collect(),
            Pattern::Accessor(a) => a.children(),
            Pattern::Call(c) => args_children(&c.args),
            Pattern::Regex(r) => {
                if let RegexLike::Pattern(p) = &r.regex {
                    p.children()
                } else {
                    Vec::new()
                }
            }
            Pattern::File(f) => {
                let mut v = Vec::new();
                let n = f.name.children();
                let b = f.body.children();
                v.extend(n);
                v.extend(b);
                v
            }
            Pattern::Files(f) => f.pattern.children(),
            Pattern::Bubble(b) => args_children(&b.args),
            Pattern::Limit(l) => l.pattern.children(),
            Pattern::CallBuiltIn(c) => args_children(&c.args),
            Pattern::CallFunction(c) => args_children(&c.args),
            Pattern::CallForeignFunction(c) => args_children(&c.args),
            Pattern::Assignment(a) => vec![PatternOrPredicate::Pattern(&a.pattern)],
            Pattern::Accumulate(a) => vec![
                PatternOrPredicate::Pattern(&a.left),
                PatternOrPredicate::Pattern(&a.right),
            ],
            Pattern::And(a) => patterns_children(&a.patterns),
            Pattern::Or(o) => patterns_children(&o.patterns),
            Pattern::Maybe(m) => vec![PatternOrPredicate::Pattern(&m.pattern)],
            Pattern::Any(a) => patterns_children(&a.patterns),
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
            Pattern::Dynamic(d) => d.children(),
            Pattern::CodeSnippet(c) => {
                let mut v = Vec::new();
                let p = c.patterns.iter().map(|p| PatternOrPredicate::Pattern(&p.1));
                let d = c
                    .dynamic_snippet
                    .as_ref()
                    .map(|d| d.children())
                    .unwrap_or(Vec::new());
                v.extend(p);
                v.extend(d);
                v
            }
            Pattern::Variable(_) => Vec::new(),
            Pattern::Rewrite(r) => {
                let mut res = r.right.children();
                res.push(PatternOrPredicate::Pattern(&r.left));
                res
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
