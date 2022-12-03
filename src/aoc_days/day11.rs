use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use cgmath::Vector2;
use itertools::Itertools;

use crate::grid::{Grid, Point};

const NEIGHBOURS: [Vector2<i32>; 8] = [
    Vector2 { y: -1, x: -1 },
    Vector2 { y: -1, x: 0 },
    Vector2 { y: -1, x: 1 },
    Vector2 { y: 0, x: -1 },
    Vector2 { y: 0, x: 1 },
    Vector2 { y: 1, x: -1 },
    Vector2 { y: 1, x: 0 },
    Vector2 { y: 1, x: 1 },
];

fn step(grid: &mut Grid<u8>) -> usize {
    let mut flashing: VecDeque<Point> = VecDeque::new();
    let mut flashed: HashSet<Point> = HashSet::new();

    // increment everyone
    for p in grid.points() {
        if let Some(x) = grid.get_mut(p) {
            *x += 1;
            if *x == 10 {
                flashing.push_back(p)
            }
        }
    }

    while let Some(p) = flashing.pop_front() {
        if flashed.contains(&p) {
            continue;
        }

        flashed.insert(p);

        for n in NEIGHBOURS {
            if let Some(x) = grid.get_mut(p + n) {
                *x += 1;
                if *x == 10 {
                    flashing.push_back(p + n)
                }
            }
        }
    }

    grid.data.iter_mut().for_each(|x| {
        if *x >= 10 {
            *x = 0
        }
    });

    flashed.len()
}

pub fn day() {
    let content = fs::read_to_string("inputs/day11").expect("Couldn't find input");

    let mut grid: Grid<u8> = Grid::default();

    content.lines().for_each(|line| {
        grid.push_row(
            line.chars()
                .map(|d| d.to_string().parse::<u8>().unwrap())
                .collect_vec(),
        )
    });

    let mut p1 = 0;
    let mut p2 = 0;

    for s in 0..100 {
        let flashed = step(&mut grid);
        p1 += flashed;
        if flashed == grid.data.len() {
            p2 = s;
        }
    }

    if p2 == 0 {
        p2 = 101;
        while step(&mut grid) != grid.rows * grid.cols {
            p2 += 1
        }
    }

    println!("{} {}", p1, p2);
}
