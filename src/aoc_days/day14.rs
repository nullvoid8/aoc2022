use std::{collections::HashMap, fs};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    sequence::{pair, separated_pair},
    IResult,
};

type Pair = (char, char);

fn parse_rule(i: &str) -> IResult<&str, ((char, char), char)> {
    separated_pair(pair(anychar, anychar), tag(" -> "), anychar)(i)
}

pub fn day() {
    let content = fs::read_to_string("inputs/day14").expect("Couldn't find input");
    let mut lines = content.lines();

    let (fixup, mut pairs) = {
        let line = lines.next().unwrap();
        let vec = line.chars().collect_vec();
        (
            (*vec.first().unwrap(), *vec.last().unwrap()),
            line.chars().tuple_windows::<(_, _)>().counts(),
        )
    };

    lines.next();

    let rules = lines
        .map(parse_rule)
        .map(|rule| rule.unwrap().1)
        .collect::<HashMap<_, _>>();

    for _ in 0..10 {
        pairs = step(&rules, pairs);
    }

    let p1 = count_chars(&pairs, fixup);

    for _ in 10..40 {
        pairs = step(&rules, pairs);
    }

    let p2 = count_chars(&pairs, fixup);

    print!("{} {}", p1, p2);
}

fn step(rules: &HashMap<Pair, char>, pairs: HashMap<Pair, usize>) -> HashMap<Pair, usize> {
    let mut out = HashMap::new();
    for ((a, b), count) in pairs.into_iter() {
        match rules.get(&(a, b)) {
            Some(&i) => {
                *out.entry((a, i)).or_default() += count;
                *out.entry((i, b)).or_default() += count;
            }
            None => {
                *out.entry((a, b)).or_default() += count;
            }
        }
    }
    out
}

fn count_chars(pairs: &HashMap<Pair, usize>, fixup: Pair) -> usize {
    let mut counts = HashMap::new();
    counts.insert(fixup.0, 1);
    counts.insert(fixup.1, 1);
    for ((a, b), c) in pairs.iter() {
        *counts.entry(*a).or_default() += *c;
        *counts.entry(*b).or_default() += *c;
    }
    let mut counts = counts.into_values().collect_vec();
    counts.sort();
    (*counts.last().unwrap() - *counts.first().unwrap()) / 2
}
