// This file does not get checked by rust as it does not participate in the module tree

use std::{cmp::max, collections::HashSet};

use inpt::{inpt, Inpt};
use itertools::Itertools;

#[derive(Debug, Inpt, Copy, Clone)]
#[inpt(regex = r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")]
pub struct Reading {
    s_x: i64,
    s_y: i64,
    b_x: i64,
    b_y: i64,
    #[inpt(skip)]
    distance: u64,
}

impl Reading {
    fn range_on_row(&self, row: i64) -> Option<(i64, i64)> {
        let dist = self.s_y.abs_diff(row);
        if dist > self.distance {
            return None;
        }

        let spread = (self.distance - dist) as i64;
        Some(((self.s_x - spread), (self.s_x + spread)))
    }
}

type Input = Vec<Reading>;

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    let mut sensor_readings: Vec<Reading> = inpt(&input).unwrap();
    sensor_readings.iter_mut().for_each(|sr| {
        sr.distance = sr.s_x.abs_diff(sr.b_x) + sr.s_y.abs_diff(sr.b_y);
    });

    Ok(sensor_readings)
}

const P1_ROW: i64 = 2000000;
const P2_START: i64 = 0;
const P2_END: i64 = 4000000;

pub fn run(input: Input) -> () {
    let mut p1 = HashSet::new();
    for sr in &input {
        if let Some(range) = sr.range_on_row(P1_ROW) {
            p1.extend(range.0..=range.1);
        }
    }

    for &Reading { b_x, b_y, .. } in &input {
        if b_y == P1_ROW {
            p1.remove(&b_x);
        }
    }

    println!("{:?}", p1.len());

    for i in P2_START..=P2_END {
        let mut ranges = input
            .iter()
            .filter_map(|sr| sr.range_on_row(i))
            .filter(|r| P2_START <= r.1 && r.0 <= P2_END)
            .collect_vec();
        ranges.sort();

        let mut here = P2_START;
        for &(s, e) in &ranges {
            if here < s {
                break;
            }
            here = max(here, e);
        }

        if here < P2_END {
            println!("{}", (here + 1) * P2_END + i);
            break;
        }
    }
}
