use crate::{context::QueryContext, pattern::EffectRange};
use grit_util::EffectKind;
use std::{cmp::Ordering, ops::Range};

pub trait Interval {
    fn interval(&self) -> (u32, u32);
}

impl Interval for Range<u32> {
    fn interval(&self) -> (u32, u32) {
        (self.start, self.end)
    }
}

fn compare<T>(i1: &T, i2: &T) -> Ordering
where
    T: Interval,
{
    let i1 = i1.interval();
    let i2 = i2.interval();
    if i1.1 < i2.1 {
        return Ordering::Less;
    }
    if i1.1 > i2.1 {
        return Ordering::Greater;
    }
    if i1.0 > i2.0 {
        return Ordering::Less;
    }
    if i1.0 < i2.0 {
        return Ordering::Greater;
    }
    Ordering::Equal
}

pub fn earliest_deadline_sort<T>(list: &mut [T]) -> bool
where
    T: Interval,
{
    list.sort_by(|b1, b2| compare(b1, b2));
    for pair in list.windows(2) {
        let p0 = pair[0].interval();
        let p1 = pair[1].interval();
        if p1.0 < p0.1 && p1.0 > p0.0 {
            return false;
        }
    }
    true
}

pub fn get_top_level_intervals<T>(effects: Vec<T>) -> Vec<T>
where
    T: Interval,
{
    let mut top_level = Vec::with_capacity(effects.len());
    let mut top_level_open = u32::MAX;
    for e in effects.into_iter().rev() {
        let e_interval = e.interval();
        if e_interval.1 <= top_level_open {
            top_level.push(e);
            top_level_open = e_interval.0;
        }
    }
    top_level
}

pub fn get_top_level_intervals_in_range<Q: QueryContext>(
    effects: Vec<EffectRange<Q>>,
    left: u32,
    right: u32,
) -> Vec<EffectRange<Q>> {
    let mut top_level = Vec::with_capacity(effects.len());
    let mut top_level_open = right;
    for e in effects.into_iter().rev() {
        let e_interval = e.interval();
        if e_interval.1 < left {
            break;
        }
        if matches!(e.effect.kind, EffectKind::Insert)
            && e_interval.0 >= left
            && e_interval.1 <= right
        {
            top_level.push(e);
            continue;
        }
        if e_interval.1 <= top_level_open && e_interval.0 >= left {
            top_level.push(e);
            top_level_open = e_interval.0;
        }
    }
    top_level
}

pub fn pop_out_of_range_intervals<T>(interval: &T, intervals: &mut Vec<T>)
where
    T: Interval,
{
    let interval = interval.interval();
    while let Some(top) = intervals.last() {
        let top_interval = top.interval();
        if top_interval.0 < interval.1 {
            break;
        }
        intervals.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::intervals::get_top_level_intervals;

    use super::earliest_deadline_sort;

    #[derive(Clone, Debug)]
    struct TestEffect {
        interval: (u32, u32),
    }
    impl TestEffect {
        fn new(interval: (u32, u32)) -> Self {
            Self { interval }
        }
    }
    impl super::Interval for TestEffect {
        fn interval(&self) -> (u32, u32) {
            self.interval
        }
    }

    impl From<(u32, u32)> for TestEffect {
        fn from(interval: (u32, u32)) -> Self {
            Self::new(interval)
        }
    }

    fn vec_into(v: &[(u32, u32)]) -> Vec<TestEffect> {
        v.iter().map(|e| TestEffect::from(*e)).collect()
    }
    fn vec_back<T>(v: &[T]) -> Vec<(u32, u32)>
    where
        T: super::Interval,
    {
        v.iter().map(|e| e.interval()).collect()
    }

    #[test]
    fn test_simple() {
        let mut list = vec_into(&[(0, 1), (1, 2), (2, 3), (3, 4)]);
        assert!(earliest_deadline_sort(&mut list));
        let list = get_top_level_intervals(list);
        println!("{:?}", vec_back(&list));
    }

    #[test]
    fn test_reverse() {
        let mut list = vec_into(&[(3, 4), (2, 3), (1, 2), (0, 1)]);
        assert!(earliest_deadline_sort(&mut list));
        let list = get_top_level_intervals(list);
        println!("{:?}", vec_back(&list));
    }

    #[test]
    fn test_nested_left() {
        let mut list = vec_into(&[(0, 1), (0, 2), (0, 3), (0, 4)]);
        assert!(earliest_deadline_sort(&mut list));
        let list = get_top_level_intervals(list);
        println!("{:?}", vec_back(&list));
        assert!(vec_back(&list) == vec![(0, 4)])
    }

    #[test]
    fn test_nested_right() {
        let mut list = vec_into(&[(1, 5), (2, 5), (3, 5), (4, 5)]);
        assert!(earliest_deadline_sort(&mut list));
        let list = get_top_level_intervals(list);
        println!("{:?}", vec_back(&list));
        assert!(vec_back(&list) == vec![(1, 5)])
    }

    #[test]
    fn overlapping_intervals() {
        let mut list = vec_into(&[(0, 1), (0, 2), (1, 2), (1, 3)]);
        assert!(!earliest_deadline_sort(&mut list));
    }

    #[test]
    fn another_overlap() {
        let mut list = vec_into(&[(0, 1), (0, 2), (1, 2), (1, 3), (2, 3), (2, 4)]);
        assert!(!earliest_deadline_sort(&mut list));
    }

    #[test]
    fn multiple_top_level_intervals() {
        let mut list = vec_into(&[(0, 1), (2, 5), (0, 2), (2, 4), (3, 4), (1, 2), (2, 3)]);
        assert!(earliest_deadline_sort(&mut list));
        let list = get_top_level_intervals(list);
        println!("{:?}", vec_back(&list));
        assert!(vec_back(&list) == vec![(2, 5), (0, 2)])
    }
}
