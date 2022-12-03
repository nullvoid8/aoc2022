use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::eof,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

fn parse_cave(i: &str) -> IResult<&str, &str> {
    alpha1(i)
}

fn parse_link(i: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(parse_cave, tag("-"), parse_cave)(i)
}

fn parse_input(i: &str) -> IResult<&str, Vec<(&str, &str)>> {
    terminated(separated_list1(line_ending, parse_link), eof)(i)
}

fn is_small_cave(name: &str) -> bool {
    name.chars().all(|c| c.is_lowercase())
}

pub fn day() {
    let content = fs::read_to_string("inputs/day12").expect("Couldn't find input");

    // map from current_cave -> Set of immediately reachable Caves
    let mut map: HashMap<_, HashSet<_>> = HashMap::new();

    for (left, right) in parse_input(&content).finish().unwrap().1 {
        // Can't point out of end / into start
        if left != "end" && right != "start" {
            map.entry(left).or_default().insert(right);
        }
        // All links (except s&e are symmetric)
        if left != "start" && right != "end" {
            map.entry(right).or_default().insert(left);
        }
    }
    let map = map;

    struct State<'a> {
        // The current cave
        here: &'a str,
        // Have/are we traversing a small cave for the second time?
        doubled: bool,
        // which small caves have we visted?
        seen: HashSet<&'a str>,
    }

    // Queue of interesting caves
    let mut frontier: VecDeque<State> = VecDeque::new();
    frontier.push_back(State {
        here: "start",
        doubled: false,
        seen: HashSet::new(),
    });

    // #paths which never revisit small caves
    let mut p1 = 0;
    // #paths which revisit one small cave
    let mut p2 = 0;

    while let Some(State {
        here,
        doubled,
        mut seen,
    }) = frontier.pop_front()
    {
        if here == "end" {
            if !doubled {
                p1 += 1;
            }
            p2 += 1;
            continue;
        }

        if is_small_cave(here) {
            seen.insert(here);
        }

        // visit all caves that aren't small caves we've already visited
        for &link in map[here].difference(&seen) {
            frontier.push_back(State {
                here: link,
                doubled,
                seen: seen.clone(),
            })
        }

        // revisit any small caves we've already been to, and prevent this branch from running again.
        if !doubled {
            for &link in map[here].intersection(&seen) {
                frontier.push_back(State {
                    here: link,
                    doubled: true,
                    seen: seen.clone(),
                })
            }
        }
    }

    println!("{} {}", p1, p2);
}
