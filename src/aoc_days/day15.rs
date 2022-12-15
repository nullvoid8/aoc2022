// This file does not get checked by rust as it does not participate in the module tree

use std::collections::HashSet;

use inpt::{inpt, Inpt};

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

type Input = Vec<Reading>;

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    let mut sensor_readings: Vec<Reading> = inpt(&input).unwrap();
    sensor_readings.iter_mut().for_each(|sr| {
        sr.distance = sr.s_x.abs_diff(sr.b_x) + sr.s_y.abs_diff(sr.b_y);
    });

    Ok(sensor_readings)
}

const P1_ROW: i64 = 2000000;

pub fn run(input: Input) -> () {
    let mut p1 = HashSet::new();
    for &Reading {
        s_x, s_y, distance, ..
    } in &input
    {
        let from_row = s_y.abs_diff(P1_ROW);
        if from_row <= distance {
            let spread = (distance - from_row) as i64;
            p1.extend((s_x - spread)..=(s_x + spread))
        }
    }

    for &Reading { b_x, b_y, .. } in &input {
        if b_y == P1_ROW {
            p1.remove(&b_x);
        }
    }

    println!("{:?}", p1.len());
}
