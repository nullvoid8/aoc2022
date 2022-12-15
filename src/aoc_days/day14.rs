use std::{
    cmp::{max, min},
    collections::HashSet,
};

use nom::{bytes::complete as bytes, character::complete as character, multi, sequence, IResult};

type Input = Vec<Path>;

type Point = (i32, i32);
type Path = Vec<Point>;

// 499,65 -> 499,68 -> 491,68 -> 491,74 -> 508,74 -> 508,68 -> 504,68 -> 504,65

fn point(i: &str) -> IResult<&str, Point> {
    sequence::separated_pair(character::i32, bytes::tag(","), character::i32)(i)
}

fn path(i: &str) -> IResult<&str, Path> {
    multi::separated_list1(bytes::tag(" -> "), point)(i)
}

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    let res = multi::separated_list1(character::line_ending, path)(&input);
    Ok(res.unwrap().1)
}

pub fn run(input: Input) -> () {
    let mut rocks = HashSet::new();
    let mut lowest = 0;

    for path in input {
        for slice in path.windows(2) {
            if let [a, b] = slice {
                lowest = max(max(lowest, a.1), b.1);
                match a.0 == b.0 {
                    // horizontal
                    true => {
                        let top = min(a.1, b.1);
                        let bot = max(a.1, b.1);
                        rocks.extend((top..=bot).map(|y| (a.0, y)))
                    }
                    // vertical
                    false => {
                        let left = min(a.0, b.0);
                        let right = max(a.0, b.0);
                        rocks.extend((left..=right).map(|x| (x, a.1)))
                    }
                }
            }
        }
    }

    let mut sand = HashSet::new();

    'outer: loop {
        let mut p = (500, 0);

        'inner: loop {
            let next = (p.0, p.1 + 1);

            if next.1 == lowest + 2 {
                sand.insert(p.clone());
                break 'inner;
            }

            if !rocks.contains(&next) && !sand.contains(&next) {
                p = next;
                continue 'inner;
            }

            let next = (p.0 - 1, p.1 + 1);

            if !rocks.contains(&next) && !sand.contains(&next) {
                p = next;
                continue 'inner;
            }

            let next = (p.0 + 1, p.1 + 1);

            if !rocks.contains(&next) && !sand.contains(&next) {
                p = next;
                continue 'inner;
            }

            sand.insert(p.clone());

            if p.1 == 0 {
                break 'outer;
            }

            break 'inner;
        }
    }

    println!("{:?}", sand.len());
}
