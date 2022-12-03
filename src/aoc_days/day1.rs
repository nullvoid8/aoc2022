use std::fs;

use itertools::Itertools;

pub fn day() {
    let contents = fs::read_to_string("inputs/day1").expect("Couldn't find input");
    let contents = contents.lines().map(|x| x.parse::<i32>().unwrap());

    let part1: i32 = contents
        .clone()
        .tuple_windows::<(_, _)>()
        .map(|(x, y)| (x < y) as i32)
        .sum();

    let part2: i32 = contents
        .tuple_windows::<(_, _, _)>()
        .tuple_windows::<(_, _)>()
        .map(|((x, _, _), (_, _, y))| (x < y) as i32)
        .sum();

    println!("{} {}", part1, part2);
}
