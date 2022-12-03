use std::fs;

use nom::{
    character::complete::{char, i32},
    multi::separated_list1,
    IResult,
};

fn parse_list(i: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(char(','), i32)(i)
}

pub fn day() {
    let content = fs::read_to_string("inputs/day7").expect("Couldn't find input");
    let mut crabs = parse_list(&content).unwrap().1;
    crabs.sort();
    let crabs = crabs;

    let min = *crabs.first().unwrap();
    let max = *crabs.last().unwrap();

    let p1 = (min..=max)
        .map(|pos| crabs.iter().map(|crab| (*crab - pos).abs()).sum::<i32>())
        .min()
        .unwrap();

    let p2 = (min..=max)
        .map(|pos| {
            crabs
                .iter()
                .map(|crab| {
                    let dist = (*crab - pos as i32).abs();
                    dist * (dist + 1) / 2
                })
                .sum::<i32>()
        })
        .min()
        .unwrap();

    println!("{} {}", p1, p2);
}
