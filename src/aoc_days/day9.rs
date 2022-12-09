use std::{cmp, collections::HashSet};

use inpt::{inpt, Inpt};
use itertools::Itertools;

#[derive(Debug, Inpt)]
pub enum Direction {
    #[inpt(regex = "U")]
    U,
    #[inpt(regex = "R")]
    R,
    #[inpt(regex = "D")]
    D,
    #[inpt(regex = "L")]
    L,
}

#[derive(Debug, Inpt)]
#[inpt(regex = r"(.) (\d+)")]
pub struct Command {
    direction: Direction,
    distance: i32,
}

type Input = Vec<Command>;

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    Ok(input.lines().map(|s| inpt(s).unwrap()).collect_vec())
}

type Point = (i32, i32);

pub fn run(input: Input) -> () {
    let mut p1_visited: HashSet<Point> = HashSet::new();
    let mut p2_visited: HashSet<Point> = HashSet::new();

    let mut rope: [Point; 10] = [(0, 0); 10];
    for c in input {
        for _ in 0..c.distance {
            match c.direction {
                Direction::U => rope[0].1 += 1,
                Direction::R => rope[0].0 += 1,
                Direction::D => rope[0].1 -= 1,
                Direction::L => rope[0].0 -= 1,
            };

            for i in 1..10 {
                if cmp::max(
                    rope[i - 1].0.abs_diff(rope[i].0),
                    rope[i - 1].1.abs_diff(rope[i].1),
                ) >= 2
                {
                    rope[i].0 += (rope[i - 1].0 - rope[i].0).signum();
                    rope[i].1 += (rope[i - 1].1 - rope[i].1).signum();
                };
            }

            p1_visited.insert(rope[1]);
            p2_visited.insert(rope[9]);
        }
    }

    println!("{}", p1_visited.len());
    println!("{}", p2_visited.len());
}
