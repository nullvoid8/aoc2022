use cgmath::Vector2;
use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    fs,
    num::ParseIntError,
    str::FromStr,
};

use crate::grid::{Grid, Point};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Height(i32);

impl Default for Height {
    fn default() -> Self {
        Height(9)
    }
}

impl FromStr for Height {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.parse::<i32>()?;
        Ok(Height(x))
    }
}

impl From<Height> for i32 {
    fn from(Height(x): Height) -> Self {
        x
    }
}

pub fn day() {
    let neighbours: Vec<_> = vec![
        Vector2 { y: -1, x: 0 },
        Vector2 { y: 1, x: 0 },
        Vector2 { y: 0, x: -1 },
        Vector2 { y: 0, x: 1 },
    ];

    let content = fs::read_to_string("inputs/day9").expect("Couldn't find input");

    // let mut grid: Grid<i32> = grid![];
    let mut grid: Grid<Height> = Grid {
        rows: 0,
        cols: 0,
        data: Vec::new(),
    };

    content.lines().for_each(|line| {
        grid.push_row(
            line.chars()
                .map(|d| d.to_string().parse::<Height>().unwrap())
                .collect_vec(),
        )
    });

    let mut low_points: Vec<Point> = Vec::new();

    for row in 0..grid.rows as i32 {
        for col in 0..grid.cols as i32 {
            let curr = Point { y: row, x: col };
            let curr_height = grid.get(curr).copied().unwrap_or_default();

            let low = neighbours
                .iter()
                .all(|&p| curr_height < grid.get(curr + p).copied().unwrap_or_default());

            if low {
                low_points.push(curr);
            }
        }
    }

    let p1: i32 = low_points
        .iter()
        .map::<i32, _>(|&p| {
            let x: i32 = grid.get(p).copied().unwrap_or_default().into();
            x + 1
        })
        .sum();

    let mut sizes: Vec<i32> = Vec::with_capacity(low_points.len());

    for init in low_points {
        let mut seen: HashSet<Point> = HashSet::new();
        let mut next: VecDeque<Point> = VecDeque::new();
        next.push_back(init);
        let mut size = 0;

        while let Some(p) = next.pop_front() {
            if seen.contains(&p) {
                continue;
            }

            seen.insert(p);
            if grid.get(p).copied().unwrap_or_default() == Height(9) {
                continue;
            }

            size += 1;
            next.extend(neighbours.iter().map(|&n| p + n))
        }

        sizes.push(size);
    }

    sizes.sort();
    sizes.reverse();

    let p2 = sizes[0] * sizes[1] * sizes[2];

    println!("{} {}", p1, p2);
}
