#![allow(warnings)]
use crate::intervals::{
    earliest_deadline_sort, get_top_level_intervals, pop_out_of_range_intervals, Interval,
};
use anyhow::{bail, Result};
use std::{
    collections::{HashMap, HashSet},
    vec,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FileInterval {
    file: String,
    start: u32,
    end: u32,
}

impl Interval for FileInterval {
    fn interval(&self) -> (u32, u32) {
        (self.start, self.end)
    }
}

impl FileInterval {
    fn new(file: String, start: u32, end: u32) -> Self {
        Self { file, start, end }
    }
}

#[derive(Debug, Clone)]
pub struct EffectInterval {
    left: FileInterval,
    right: Vec<FileInterval>,
}

impl Interval for EffectInterval {
    fn interval(&self) -> (u32, u32) {
        (self.left.start, self.left.end)
    }
}

impl EffectInterval {
    fn new(left: FileInterval, right: Vec<FileInterval>) -> Self {
        Self { left, right }
    }
}

pub trait ToEffectInterval {
    fn to_effect_interval(&self) -> EffectInterval;
}

pub struct EffectsInfo<T: ToEffectInterval + Clone> {
    interval_map: HashMap<FileInterval, (T, Vec<FileInterval>)>,
    file_to_effect_intervals: HashMap<String, Vec<T>>,
    rhs_to_lhs: HashMap<FileInterval, Vec<FileInterval>>,
    file_to_sorted_intervals: HashMap<String, Vec<FileInterval>>,
}

// takes a vectore of effects, and returns relevant datastructures
// Map from lhs of effect as interval to effect
// Map from file to EffectIntervals
// Map from rhs intervals to corresponding lhs Interval
fn effects_to_intervals<T>(effects: Vec<T>) -> Result<EffectsInfo<T>>
where
    T: ToEffectInterval + Clone,
{
    let mut interval_map: HashMap<FileInterval, (T, Vec<FileInterval>)> = HashMap::new();
    let mut file_to_effect_intervals: HashMap<String, Vec<T>> = HashMap::new();
    let mut rhs_to_lhs: HashMap<FileInterval, Vec<FileInterval>> = HashMap::new();
    let mut file_to_sorted_intervals: HashMap<String, HashSet<FileInterval>> = HashMap::new();
    for effect in effects {
        let effect_interval = effect.to_effect_interval();
        let lhs = effect_interval.left;
        let rhs = effect_interval.right;
        let file = lhs.file.clone();
        let old = interval_map.insert(lhs.clone(), (effect.clone(), rhs.clone()));
        if old.is_some() {
            bail!("duplicate lhs interval");
        }
        file_to_effect_intervals
            .entry(file.to_owned())
            .or_insert_with(std::vec::Vec::new)
            .push(effect.clone());
        file_to_sorted_intervals
            .entry(file.to_owned())
            .or_insert_with(std::collections::HashSet::new)
            .insert(lhs.clone());
        for interval in rhs.clone() {
            file_to_sorted_intervals
                .entry(file.to_owned())
                .or_insert_with(std::collections::HashSet::new)
                .insert(interval.clone());
        }
        for interval in rhs {
            rhs_to_lhs
                .entry(interval)
                .or_insert_with(std::vec::Vec::new)
                .push(lhs.clone());
        }
    }
    let mut file_to_sorted_intervals: HashMap<String, Vec<FileInterval>> = file_to_sorted_intervals
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect();
    for intervals in file_to_sorted_intervals.values_mut() {
        if !earliest_deadline_sort(intervals) {
            bail!("effects have overlapping lhs intervals");
        }
    }
    let res = EffectsInfo {
        interval_map,
        file_to_effect_intervals,
        rhs_to_lhs,
        file_to_sorted_intervals,
    };
    Ok(res)
}

// takes a vector of EffectIntervals and returns a vector of EffectIntervals
// whose lhs are not contained in any other lhs
fn filter_top_level_effects(effects: &mut [EffectInterval]) -> Result<Vec<EffectInterval>> {
    if !earliest_deadline_sort(effects) {
        bail!("effects have overlapping lhs intervals");
    }
    Ok(get_top_level_intervals(effects.to_vec()))
}

fn top_level_effects_from_all_files(
    effects: &mut HashMap<String, Vec<EffectInterval>>,
) -> Result<Vec<EffectInterval>> {
    let res: Result<Vec<Vec<EffectInterval>>> = effects
        .values_mut()
        .map(|es| filter_top_level_effects(es))
        .collect();
    let res = res?;
    Ok(res.into_iter().flatten().collect())
}

pub fn get_effects_order<T>(effects: Vec<T>) -> Result<(EffectsInfo<T>, Vec<FileInterval>)>
where
    T: ToEffectInterval + Clone,
{
    let info = effects_to_intervals(effects)?;
    let lhs_intervals = info.interval_map.keys().cloned().collect::<Vec<_>>();
    let by_file = &info.file_to_sorted_intervals;
    for (file, sorted) in by_file {
        println!("file: {}", file);
        let to_print = sorted.iter().map(|i| (i.start, i.end)).collect::<Vec<_>>();
        println!("intervals: {:?}", to_print)
    }
    let graph = build_dependency_graph(&lhs_intervals, by_file, &info.rhs_to_lhs);
    let linearized = linearize_graph(graph)?;
    Ok((info, linearized))
}

fn build_dependency_graph(
    effects: &[FileInterval],
    by_file: &HashMap<String, Vec<FileInterval>>,
    rhs_to_lhs: &HashMap<FileInterval, Vec<FileInterval>>,
) -> HashMap<FileInterval, HashSet<FileInterval>> {
    let mut map = effects
        .iter()
        .map(|e| (e.to_owned(), HashSet::new()))
        .collect::<HashMap<FileInterval, HashSet<FileInterval>>>();
    for intervals in by_file.values() {
        add_dependencies_for_file(intervals, &mut map, rhs_to_lhs);
    }
    map
}

// assumes intervals are already EDS sorted;
fn add_dependencies_for_file(
    intervals: &[FileInterval],
    map: &mut HashMap<FileInterval, HashSet<FileInterval>>,
    rhs_to_lhs: &HashMap<FileInterval, Vec<FileInterval>>,
) {
    let mut lhs_stack: Vec<FileInterval> = vec![];
    let mut rhs_stack: Vec<FileInterval> = vec![];
    for e in intervals.iter().rev() {
        pop_out_of_range_intervals(e, &mut lhs_stack);
        pop_out_of_range_intervals(e, &mut rhs_stack);
        // ORDER MATTERS HERE
        // if a range is both lhs and rhs, then the effects
        // corresponding to the rhs depend on the effects corresponding to the lhs
        // so pushing onto rhs_stack first ensures that the rhs effects of the interval
        // are prosseced in the event that it is also lhs.
        if let Some(sources) = rhs_to_lhs.get(e) {
            // adds all the effects corresponding which have e on the rhs
            // as dependencies of all effects with lhs enclosing e
            for lhs in lhs_stack.iter() {
                let old = map
                    .entry(lhs.clone())
                    .or_insert_with(std::collections::HashSet::new);
                old.extend(sources.clone());
            }
            rhs_stack.push(e.clone());
        }
        if map.contains_key(e) {
            // adds e as a dependency to all effects whose lhs encloses e
            for lhs in lhs_stack.iter() {
                let old = map
                    .entry(lhs.clone())
                    .or_insert_with(std::collections::HashSet::new);
                old.insert(e.clone());
            }
            // adds e as a dependency to all effects whose rhs encloses e
            for rhs in rhs_stack.iter() {
                // should always be true
                if let Some(sources) = rhs_to_lhs.get(rhs) {
                    for source in sources {
                        let old = map
                            .entry(source.clone())
                            .or_insert_with(std::collections::HashSet::new);
                        old.insert(e.clone());
                    }
                }
            }
            lhs_stack.push(e.clone());
        }
    }
}

fn linearize_graph(
    mut dependency_graph: HashMap<FileInterval, HashSet<FileInterval>>,
) -> Result<Vec<FileInterval>> {
    let mut dependants: HashMap<FileInterval, HashSet<FileInterval>> = HashMap::new();
    let mut dependency_free: Vec<FileInterval> = vec![];
    let mut linearized: Vec<FileInterval> = vec![];
    for (interval, dependencies) in &dependency_graph {
        if dependencies.is_empty() {
            dependency_free.push(interval.clone());
        }
        for dependency in dependencies {
            let old = dependants
                .entry(dependency.clone())
                .or_insert_with(std::collections::HashSet::new);
            old.insert(interval.clone());
        }
    }
    while let Some(interval) = dependency_free.pop() {
        linearized.push(interval.clone());
        if let Some(dependants) = dependants.get(&interval) {
            for dependant in dependants {
                let old = dependency_graph
                    .get_mut(dependant)
                    .expect("dependant not in dependency graph");
                old.remove(&interval);
                if old.is_empty() {
                    dependency_free.push(dependant.clone());
                }
            }
        }
    }
    if linearized.len() != dependency_graph.len() {
        bail!("dependency graph has a cycle");
    }
    Ok(linearized)
}

#[cfg(test)]
mod tests {

    use std::collections::{HashMap, HashSet};

    use crate::intervals::{earliest_deadline_sort, Interval};

    use super::{
        get_effects_order, linearize_graph, EffectInterval, FileInterval, ToEffectInterval,
    };

    type NestedVec = Vec<((u32, u32), Vec<(u32, u32)>)>;
    type NestedArray = [((u32, u32), Vec<(u32, u32)>)];

    fn vec_into(v: &[(u32, u32)]) -> Vec<FileInterval> {
        v.iter().map(interval_to_file).collect()
    }

    fn vec_to_set(v: &[(u32, u32)]) -> std::collections::HashSet<FileInterval> {
        v.iter().map(interval_to_file).collect()
    }

    fn interval_to_file(e: &(u32, u32)) -> FileInterval {
        FileInterval::new("default".to_owned(), e.0, e.1)
    }

    fn nested_vec_to_map(
        v: &NestedArray,
    ) -> std::collections::HashMap<FileInterval, Vec<FileInterval>> {
        v.iter()
            .map(|(lhs, rhs)| (interval_to_file(lhs), vec_into(rhs)))
            .collect()
    }

    #[allow(dead_code)]
    fn map_to_vec(map: &HashMap<FileInterval, HashSet<FileInterval>>) -> NestedVec {
        map.iter()
            .map(|(lhs, rhs)| (lhs.interval(), rhs.iter().map(|f| f.interval()).collect()))
            .collect()
    }

    fn assert_map(map: &HashMap<FileInterval, HashSet<FileInterval>>, expected: &NestedVec) {
        let expected = nested_vec_to_map(expected)
            .iter()
            .map(|(k, v)| (k.to_owned(), v.iter().map(|e| e.to_owned()).collect()))
            .collect();
        assert_eq!(map, &expected);
    }

    #[allow(dead_code)]
    fn print_res(res: NestedVec) {
        for (lhs, rhs) in res {
            println!("{:?} -> {:?}", lhs, rhs);
        }
    }

    fn init_map_from_vec(
        lhs_intervals: &[(u32, u32)],
    ) -> HashMap<FileInterval, HashSet<FileInterval>> {
        let lhs_intervals = vec_to_set(lhs_intervals);
        let mut map = HashMap::new();
        for lhs in lhs_intervals {
            map.insert(lhs.clone(), HashSet::new());
        }
        map
    }

    fn dependency_tester(
        intervals: &mut [(u32, u32)],
        map: &mut HashMap<FileInterval, HashSet<FileInterval>>,
        rhs_to_lhs: &NestedArray,
    ) {
        let mut intervals = vec_into(intervals);
        assert!(earliest_deadline_sort(&mut intervals));
        let rhs_to_lhs = nested_vec_to_map(rhs_to_lhs);
        super::add_dependencies_for_file(&intervals, map, &rhs_to_lhs);
    }

    #[test]
    fn no_nesting_test() {
        let intervals = &mut [(0, 1), (2, 3), (4, 5), (6, 7)];
        let lhs_intervals = &[(0, 1), (2, 3), (4, 5), (6, 7)];
        let rhs_to_lhs = &[
            ((0, 1), vec![(2, 3), (4, 5), (6, 7)]),
            ((2, 3), vec![(4, 5), (6, 7)]),
            ((4, 5), vec![(6, 7)]),
        ];
        let mut map = init_map_from_vec(lhs_intervals);
        dependency_tester(intervals, &mut map, rhs_to_lhs);
        let expected = vec![
            ((6, 7), vec![(0, 1), (2, 3), (4, 5)]),
            ((2, 3), vec![(0, 1)]),
            ((4, 5), vec![(0, 1), (2, 3)]),
            ((0, 1), vec![]),
        ];
        assert_map(&map, &expected);
    }

    #[derive(Debug, Clone)]
    struct EffectIntervalTest {
        left: FileInterval,
        right: Vec<FileInterval>,
    }

    impl EffectIntervalTest {
        fn new(left: FileInterval, right: Vec<FileInterval>) -> Self {
            Self { left, right }
        }
    }

    fn default_file_effect(l: (u32, u32), r: &[(u32, u32)]) -> EffectIntervalTest {
        let left = FileInterval::new("default".to_owned(), l.0, l.1);
        let right = r
            .iter()
            .map(|e| FileInterval::new("default".to_owned(), e.0, e.1))
            .collect();
        EffectIntervalTest::new(left, right)
    }

    fn default_file_array(intervals: &NestedArray) -> Vec<EffectIntervalTest> {
        intervals
            .iter()
            .map(|(l, r)| default_file_effect(*l, r))
            .collect()
    }

    impl ToEffectInterval for EffectIntervalTest {
        fn to_effect_interval(&self) -> EffectInterval {
            let left = self.left.to_owned();
            let right = self.right.iter().map(|e| e.to_owned()).collect();
            EffectInterval::new(left, right)
        }
    }

    #[allow(dead_code)]
    fn vec_back<T>(v: &[T]) -> Vec<(u32, u32)>
    where
        T: super::Interval,
    {
        v.iter().map(|e| e.interval()).collect()
    }

    #[test]
    fn nested_intervals_lhs_only_test() {
        let intervals = &mut [(0, 1), (2, 5), (0, 2), (2, 4), (3, 4), (1, 2), (2, 3)];
        let lhs_intervals = &[(0, 1), (2, 5), (0, 2), (2, 4), (3, 4), (1, 2), (2, 3)];
        let rhs_to_lhs = &[];
        let mut map = init_map_from_vec(lhs_intervals);
        dependency_tester(intervals, &mut map, rhs_to_lhs);
        let expected = vec![
            ((0, 2), vec![(0, 1), (1, 2)]),
            ((2, 4), vec![(2, 3), (3, 4)]),
            ((2, 5), vec![(2, 3), (2, 4), (3, 4)]),
            ((3, 4), vec![]),
            ((1, 2), vec![]),
            ((2, 3), vec![]),
            ((0, 1), vec![]),
        ];
        assert_map(&map, &expected);
    }
    #[test]
    fn nested_intervals_test() {
        let intervals = &mut [(0, 1), (2, 5), (0, 2), (2, 4), (3, 4), (1, 2), (2, 3)];
        // if we were to filter out top level intervals, (0, 2) would make (0, 1) and (1, 2)
        // and we would expect to remove them from the graph.
        let lhs_intervals = &[(0, 1), (1, 2), (0, 2), (2, 3), (3, 4), (2, 5)];
        let rhs_to_lhs = &[
            ((2, 4), vec![(0, 1)]),
            ((2, 5), vec![(1, 2)]),
            ((3, 4), vec![(2, 3)]),
        ];
        let mut map = init_map_from_vec(lhs_intervals);
        dependency_tester(intervals, &mut map, rhs_to_lhs);
        let expected = vec![
            ((0, 1), vec![(2, 3), (3, 4)]),
            ((1, 2), vec![(2, 5), (3, 4), (2, 3)]),
            ((0, 2), vec![(0, 1), (1, 2)]),
            ((2, 5), vec![(0, 1), (2, 3), (3, 4)]),
            ((2, 3), vec![(3, 4)]),
            ((3, 4), vec![]),
        ];
        assert_map(&map, &expected);
        let linear = linearize_graph(map).unwrap();
        assert_eq!(
            linear,
            vec_into(&[(3, 4), (2, 3), (0, 1), (2, 5), (1, 2), (0, 2)])
        );
    }

    #[test]
    fn linearize_effects() {
        let effects = default_file_array(&[
            ((2, 5), vec![]),
            ((1, 2), vec![(2, 5)]),
            ((0, 2), vec![]),
            ((3, 4), vec![]),
            ((0, 1), vec![(2, 4)]),
            ((2, 3), vec![(3, 4)]),
        ]);
        let (_info, linear) = get_effects_order(effects).unwrap();

        assert_eq!(
            linear,
            vec_into(&[(3, 4), (2, 3), (0, 1), (2, 5), (1, 2), (0, 2)])
        );
    }
}
