use std::fs;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of, space1},
    combinator::{eof, map},
    multi::{many1, separated_list0},
    sequence::{separated_pair, terminated},
    IResult,
};

fn parse_seg(i: &str) -> IResult<&str, u8> {
    map(many1(one_of("abcdefg")), |wires| {
        wires
            .into_iter()
            .map(|wire| match wire {
                'a' => 64,
                'b' => 32,
                'c' => 16,
                'd' => 8,
                'e' => 4,
                'f' => 2,
                'g' => 1,
                _ => 0,
            })
            .sum::<u8>()
    })(i)
}

struct Readout {
    numbers: [u8; 10],
    result: [u8; 4],
}

fn parse_readout(i: &str) -> IResult<&str, Readout> {
    map(
        separated_pair(
            separated_list0(space1, parse_seg),
            tag(" | "),
            separated_list0(space1, parse_seg),
        ),
        |(numbers, result)| Readout {
            numbers: numbers.try_into().unwrap(),
            result: result.try_into().unwrap(),
        },
    )(i)
}

fn parse_input(i: &str) -> IResult<&str, Vec<Readout>> {
    terminated(separated_list0(line_ending, parse_readout), eof)(i)
}

fn decode(Readout { numbers, result }: Readout) -> i32 {
    let mut digits = [0; 10];

    // known 1 4 7 8
    digits[1] = numbers
        .iter()
        .find(|x| x.count_ones() == 2)
        .unwrap()
        .clone();
    digits[4] = numbers
        .iter()
        .find(|x| x.count_ones() == 4)
        .unwrap()
        .clone();
    digits[7] = numbers
        .iter()
        .find(|x| x.count_ones() == 3)
        .unwrap()
        .clone();
    digits[8] = numbers
        .iter()
        .find(|x| x.count_ones() == 7)
        .unwrap()
        .clone();

    //   a = 7 - 1
    // wires[0] = digits[7] - digits[1];

    // 0,6,9 are all 6 segs. only 9 contains 4, only 0,9 contain 7.
    let sixes = numbers
        .iter()
        .filter(|x| x.count_ones() == 6)
        .cloned()
        .collect_vec();
    for x in sixes {
        if x & digits[4] == digits[4] {
            digits[9] = x;
            continue;
        }
        if x & digits[7] == digits[7] {
            digits[0] = x;
            continue;
        }
        digits[6] = x;
    }

    // 2,3,5 are all 5 segs. // only 3 overlaps 1. only 2 overlaps 8-6
    let fives = numbers
        .iter()
        .filter(|x| x.count_ones() == 5)
        .cloned()
        .collect_vec();

    let wire_b = digits[8] & !digits[6];

    for x in fives {
        if x & digits[1] == digits[1] {
            digits[3] = x;
            continue;
        }
        if x & wire_b == wire_b {
            digits[2] = x;
            continue;
        }
        digits[5] = x;
    }

    let mut digit_map = [0i32; 256];

    for (i, x) in digits.into_iter().enumerate() {
        digit_map[x as usize] = i as i32
    }

    digit_map[result[0] as usize] * 1000
        + digit_map[result[1] as usize] * 100
        + digit_map[result[2] as usize] * 10
        + digit_map[result[3] as usize]
}

pub fn day() {
    let content = fs::read_to_string("inputs/day8").expect("Couldn't find input");
    let readouts = parse_input(&content).unwrap().1;

    let p1: usize = readouts
        .iter()
        .map(|Readout { result, .. }| {
            result
                .into_iter()
                .filter(|n| match n.count_ones() {
                    2 => true,
                    4 => true,
                    3 => true,
                    7 => true,
                    _ => false,
                })
                .count()
        })
        .sum();

    let p2: i32 = readouts.into_iter().map(decode).sum();

    println!("{} {}", p1, p2);
}
