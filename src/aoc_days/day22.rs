use std::{
    cmp::{max, min},
    fs,
    ops::Sub,
};

use cgmath::Point3;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, i32, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

#[derive(Debug, Clone, Copy)]
struct Cube {
    min: Point3<i32>,
    max: Point3<i32>,
}

impl Cube {
    fn intersect(self, other: Self) -> Option<Self> {
        let low = self.min.zip(other.min, max);
        let high = self.max.zip(other.max, min);
        match low.zip(high, |x, y| x <= y).into() {
            (true, true, true) => Some(Cube {
                min: low,
                max: high,
            }),
            _ => None,
        }
    }

    fn size(&self) -> u64 {
        match self.max.zip(self.min, i32::sub) {
            Point3 { x, y, z } => ((x + 1) as u64 * (y + 1) as u64 * (z + 1) as u64),
        }
    }

    fn parse(i: &str) -> IResult<&str, Cube> {
        tuple((
            preceded(tag("x="), separated_pair(i32, tag(".."), i32)),
            preceded(tag(",y="), separated_pair(i32, tag(".."), i32)),
            preceded(tag(",z="), separated_pair(i32, tag(".."), i32)),
        ))
        .map(|((lx, hx), (ly, hy), (lz, hz))| Cube {
            min: (lx, ly, lz).into(),
            max: (hx, hy, hz).into(),
        })
        .parse(i)
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Add,
    Remove,
}

impl Action {
    fn parse(i: &str) -> IResult<&str, Action> {
        terminated(alpha1, space1)
            .map(|x| match x {
                "on" => Action::Add,
                "off" => Action::Remove,
                _ => panic!("bad match"),
            })
            .parse(i)
    }
}

#[derive(Debug, Clone, Copy)]
struct Step {
    action: Action,
    cube: Cube,
}

impl Step {
    fn parse(i: &str) -> IResult<&str, Self> {
        Action::parse
            .and(Cube::parse)
            .map(|(action, cube)| Step { action, cube })
            .parse(i)
    }
}

fn parse_input(i: &str) -> IResult<&str, Vec<Step>> {
    separated_list1(line_ending, Step::parse)(i)
}

const INIT_CUBE: Cube = Cube {
    min: Point3 {
        x: -50,
        y: -50,
        z: -50,
    },
    max: Point3 {
        x: 50,
        y: 50,
        z: 50,
    },
};

pub fn day() {
    let content = fs::read_to_string("inputs/day22").expect("Couldn't find input");
    let steps = parse_input(&content).unwrap().1;

    let mut space: Space = Space::new();

    for step in steps {
        match step.action {
            Action::Add => (space).add(step.cube),
            Action::Remove => (space).remove(step.cube),
        }
        (space.size());
    }

    let p2 = space.size();
    space.restrict(INIT_CUBE);
    let p1 = space.size();

    println!("{} {}", p1, p2);
}

#[derive(Debug, Clone)]
struct Space {
    positive: Vec<Cube>,
    negative: Vec<Cube>,
}

impl Space {
    fn new() -> Self {
        Space {
            positive: Vec::default(),
            negative: Vec::default(),
        }
    }

    fn add(&mut self, new: Cube) {
        let positive = self
            .negative
            .iter()
            .filter_map(|c| c.intersect(new))
            .collect_vec();
        let negative = self
            .positive
            .iter()
            .filter_map(|c| c.intersect(new))
            .collect_vec();
        self.positive.push(new);
        self.positive.extend(positive);
        self.negative.extend(negative);
    }

    fn remove(&mut self, new: Cube) {
        let mut copy = self.clone();
        copy.restrict(new);

        self.positive.extend(copy.negative);
        self.negative.extend(copy.positive);
    }

    fn restrict(&mut self, bounds: Cube) {
        self.positive = self
            .positive
            .iter()
            .filter_map(|c| c.intersect(bounds))
            .collect_vec();
        self.negative = self
            .negative
            .iter()
            .filter_map(|c| c.intersect(bounds))
            .collect_vec();
    }

    fn size(&self) -> u64 {
        let p: u64 = self.positive.iter().map(Cube::size).sum();
        let n: u64 = self.negative.iter().map(Cube::size).sum();
        p - n
    }
}
