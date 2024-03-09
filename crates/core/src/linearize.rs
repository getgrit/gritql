use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    usize,
};

use anyhow::{bail, Result};
use marzano_util::position::ByteInterval;

// Effects are made up of a lhs interveral and rhs intervals, the lhs represents
// the interval of the lhs represents the part of the code to rewrite
// and the rhs intervals represent parts of the code to be "pasted in".
// an effect depends on effects whose interval is a subset of an interval in the lhs
// of the effect. An effect overwrites another effect if the rhs of the second effect
// is a subset of the rhs of the first effect. To linearize we first identify the top
// level effects, that is effects not overwritten by any other effect. Then we identify
// all of those effects transative dependencies, and finally we topologically sort
// the effects such that an effect only takes place after its dependencies and any
// effects it overwrites.

struct Interval {
    left: usize,
    right: usize,
    side: Side,
}

enum Side {
    Left,
    Right(Vec<Interval>),
}

// todo create a custom hasher that uses the function
// left * max + right for better performance.

// assumes eds order
fn get_top_level_effects<T>(effects: &Vec<T>, left: usize, right: usize) -> Vec<usize>
where
    T: Effect,
{
    let mut top_level = Vec::with_capacity(effects.len());
    let mut top_level_open = right;
    for (i, e) in effects.iter().enumerate().rev() {
        if e.interval().1 < top_level_open {
            if e.interval().0 < left {
                break;
            }
            top_level.push(i);
            top_level_open = e.interval().0;
        }
    }
    top_level
}

fn compare(i1: &(usize, usize), i2: &(usize, usize)) -> Ordering {
    if i1.0 < i2.0 {
        return Ordering::Less;
    }
    if i1.0 > i2.0 {
        return Ordering::Greater;
    }
    if i1.1 > i2.1 {
        return Ordering::Less;
    }
    if i1.1 < i2.1 {
        return Ordering::Greater;
    }
    Ordering::Equal
}

fn earliest_deadline_sort(list: &mut Vec<(usize, usize)>) -> bool {
    list.sort_by(|b1, b2| compare(b1, b2));
    for pair in list.windows(2) {
        if pair[0].1 > pair[1].0 {
            return false;
        }
    }
    true
}

fn get_top_level_intervals(effects: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut top_level = Vec::with_capacity(effects.len());
    let mut top_level_open = usize::MAX;
    for e in effects.iter().rev() {
        if e.1 < top_level_open {
            top_level.push(e.to_owned());
            top_level_open = e.0;
        }
    }
    top_level
}

// fn get_visible_effects(
//     top_level: Vec<(usize, usize)>,
//     dependants: HashMap<(usize, usize), Vec<(usize, usize)>>,
//     sorted_ranges: Vec<(usize, usize)>,
// ) -> HashSet<(usize, usize)> {
//     let mut visible = Vec::with_capacity(top_level.len());
//     let mut seen = HashSet::new();
//     for i in top_level.iter().rev() {
//         if seen.contains(i) {
//             continue;
//         }
//         visible.push(i.to_owned());
//         seen.insert(i);
//         if let Some(deps) = dependants.get(i) {
//             for d in deps.iter() {
//                 if seen.contains(d) {
//                     continue;
//                 }
//                 visible.push(d.to_owned());
//                 seen.insert(d);
//             }
//         }
//     }
//     visible
// }

//assumes at least one effect in eds order
fn get_effect_dependencies(
    dependants: &HashMap<(usize, usize), HashSet<(usize, usize)>>,
    sorted_ranges: &[(usize, usize)],
    effects: &HashSet<&(usize, usize)>,
) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
    let ranges = sorted_ranges.iter().rev();
    let mut stack: Vec<(usize, usize)> = vec![];
    let mut effect_deps: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    for range in ranges {
        // stack should consist of ranges current range is nested inside
        while !stack.is_empty() && stack.last().unwrap().0 > range.1 {
            stack.pop();
        }
        if effects.contains(range) {
            if let Some(deps) = dependants.get(range) {
                for d in deps.iter() {
                    effect_deps.entry(*d).or_default().insert(range.to_owned());
                }
            }
            for s in stack.iter().rev() {
                if effects.contains(s) {
                    break;
                }
                if let Some(deps) = dependants.get(range) {
                    for d in deps.iter() {
                        effect_deps.entry(*d).or_default().insert(range.to_owned());
                    }
                }
            }
        }
        stack.push(range.to_owned());
    }
    effect_deps
}

// collect all top level rhs intervals into a hashmap from rhs to list of lhs intervals
fn process_effects<T>(effects: &Vec<T>) -> Result<HashMap<Interval, Vec<Interval>>>
where
    T: Effect,
{
    let sources: HashMap<(usize, usize), &T> = effects.iter().map(|e| (e.interval(), e)).collect();
    let mut top_level = sources.iter().map(|e| e.0.to_owned()).collect();
    if !earliest_deadline_sort(&mut top_level) {
        bail!("overlapping effects");
    }
    let top_level = get_top_level_intervals(&top_level);
    let mut ranges: HashSet<(usize, usize)> = sources.keys().map(|e| e.to_owned()).collect();
    if sources.len() != effects.len() {
        bail!("cannot have multiple effects with the same interval");
    }
    let mut dependants: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    for e in effects.iter() {
        let l = e.interval();
        let r = e.intervals();
        for i in r.iter() {
            dependants.entry(*i).or_default().insert(l);
            ranges.insert(*i);
        }
    }
    let mut ranges: Vec<(usize, usize)> = ranges.into_iter().collect();
    if !earliest_deadline_sort(&mut ranges) {
        bail!("overlapping ranges");
    }
    let dependencies = get_effect_dependencies(&dependants, &ranges, &sources.keys().collect());
    // let visible = get_visible_effects(dependants, sorted_ranges);
    todo!()
}

trait Effect: ByteInterval {
    fn intervals(&self) -> Vec<(usize, usize)>;

    fn linearize<T>(mut effects: Vec<T>) -> Result<Vec<T>>
    where
        T: Effect,
    {
        if Vec::is_empty(&effects) {
            return Ok(effects);
        }
        if !T::earliest_deadline_sort(&mut effects) {
            return Err(anyhow::anyhow!("overlapping effects"));
        }

        todo!()
    }
}

/*
{
    collect lhs intervals
    get top level effects
    return top level effects
}
{
    collect all intervals
    get dependencies of top level effects
    return all effects depended upon by top level effects
}
{
    collect all ranges depended upon by top level effects
    add dependencies for dependency, override, and conflict to graph
}
{
    order effects by dependency graph
}
{
    apply effects sequentially
}
 */
