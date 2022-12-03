use std::fs;

use nom::{
    character::complete::{char, i64},
    multi::separated_list1,
    IResult,
};

fn parse(i: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(char(','), i64)(i)
}

pub fn day() {
    let content = fs::read_to_string("inputs/day6").expect("Couldn't find input");

    let (_, nums) = parse(&content).unwrap();

    let mut state = [0; 9];

    for i in nums {
        state[i as usize] += 1;
    }

    for _ in 0..80 {
        let babies = state[0];
        for i in 0..8 {
            state[i] = state[i + 1];
        }
        state[6] += babies;
        state[8] = babies;
    }

    let p1: i64 = state.iter().sum();

    for _ in 80..256 {
        let babies = state[0];
        for i in 0..8 {
            state[i] = state[i + 1];
        }
        state[6] += babies;
        state[8] = babies;
    }

    let p2: i64 = state.iter().sum();

    println!("{} {}", p1, p2);
}
