use std::{collections::HashMap, fs, iter::successors};

use gcd::Gcd;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, i32, line_ending},
    combinator::{eof, map},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

type Point = (i32, i32);

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    separated_pair(i32, char(','), i32)(input)
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    map(
        separated_pair(parse_point, tag(" -> "), parse_point),
        |(start, end)| Line { start, end },
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Line>> {
    terminated(separated_list1(line_ending, parse_line), eof)(input)
}

fn gen_points(line: Line) -> Vec<Point> {
    let Line { start, end } = line;

    let step = (end.0 - start.0, end.1 - start.1);
    let scale: i32 = (step.0.unsigned_abs())
        .gcd(step.1.unsigned_abs())
        .try_into()
        .unwrap();
    let step = (step.0 / scale, step.1 / scale);

    successors(Some(start), |curr| {
        if *curr == end {
            return None;
        }
        Some((curr.0 + step.0, curr.1 + step.1))
    })
    .collect_vec()
}

fn is_straight(line: &Line) -> bool {
    line.start.0 == line.end.0 || line.start.1 == line.end.1
}

pub fn day() {
    let content = fs::read_to_string("inputs/day5").expect("Couldn't find input");
    let (_, lines) = parse(&content).unwrap();

    let mut seen_points: HashMap<Point, i32> = HashMap::new();

    let (straights, angles) = lines.into_iter().partition::<Vec<_>, _>(is_straight);

    straights
        .into_iter()
        .flat_map(gen_points)
        .for_each(|p| *(seen_points.entry(p).or_default()) += 1);

    let p1 = seen_points.values().filter(|&&c| c > 1).count();

    angles
        .into_iter()
        .flat_map(gen_points)
        .for_each(|p| *(seen_points.entry(p).or_default()) += 1);

    let p2 = seen_points.values().filter(|&&c| c > 1).count();

    println!("{} {}", p1, p2);
}
