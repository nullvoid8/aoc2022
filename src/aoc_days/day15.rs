use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs,
};

use cgmath::Vector2;
use itertools::Itertools;

use crate::grid::{Grid, Point};

pub fn day() {
    let content = fs::read_to_string("inputs/day15").expect("Couldn't find input");
    let grid: Grid<_> = {
        let mut grid = Grid::default();
        content.lines().for_each(|line| {
            grid.push_row(
                line.chars()
                    .map(|d| d.to_string().parse::<usize>().unwrap() - 1)
                    .collect_vec(),
            )
        });
        grid
    };

    let p1 = search(&grid).map(|r| r.cost);
    let p2 = search_large(&grid).map(|r| r.cost);

    println!("{:?} {:?}", p1, p2);
}

#[derive(Debug)]
struct State {
    here: Point,
    path: Vec<Point>,
    cost: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Reverse(self.cost).partial_cmp(&Reverse(other.cost))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.cost).cmp(&Reverse(other.cost))
    }
}

const NEIGHBOURS: [Vector2<i32>; 4] = [
    Vector2 { y: -1, x: 0 },
    Vector2 { y: 0, x: -1 },
    Vector2 { y: 0, x: 1 },
    Vector2 { y: 1, x: 0 },
];

fn search(grid: &Grid<usize>) -> Option<State> {
    let mut seen: HashSet<Point> = HashSet::new();
    let mut frontier: BinaryHeap<State> = BinaryHeap::from([State {
        here: (0, 0).into(),
        path: vec![(0, 0).into()],
        cost: 0,
    }]);

    let goal: Point = (grid.cols as i32 - 1, grid.rows as i32 - 1).into();

    while let Some(s @ State { here, cost, .. }) = frontier.pop() {
        if here == goal {
            return Some(s);
        }
        if seen.contains(&here) {
            continue;
        }
        seen.insert(here);

        for n in NEIGHBOURS {
            let next = here + n;

            if let Some(danger) = grid.get(next) {
                frontier.push(State {
                    here: next,
                    cost: cost + danger + 1,
                    path: {
                        let mut path = s.path.clone();
                        path.push(next);
                        path
                    },
                })
            }
        }
    }

    None
}

fn search_large(grid: &Grid<usize>) -> Option<State> {
    let mut seen: HashSet<Point> = HashSet::new();
    let mut frontier: BinaryHeap<State> = BinaryHeap::from([State {
        here: (0, 0).into(),
        path: vec![(0, 0).into()],
        cost: 0,
    }]);

    let goal = (5 * Point::from((grid.cols as i32, grid.rows as i32))) - Vector2::from((1, 1));

    while let Some(s @ State { here, cost, .. }) = frontier.pop() {
        if here == goal {
            return Some(s);
        }
        if seen.contains(&here) {
            continue;
        }
        seen.insert(here);

        for n in NEIGHBOURS {
            let next = here + n;

            if !(0 <= next.x && next.x < (grid.cols as i32 * 5)) {
                continue;
            }
            if !(0 <= next.y && next.y < (grid.cols as i32 * 5)) {
                continue;
            }

            let next_base = next.zip((grid.cols, grid.rows).into(), |a, b| a % b as i32);
            let extra = next.zip((grid.cols, grid.rows).into(), |a, b| a as usize / b);

            let extra = extra.x + extra.y;

            if let Some(danger) = grid.get(next_base) {
                frontier.push(State {
                    here: next,
                    cost: cost + (((danger + extra) % 9) + 1),
                    path: {
                        let mut path = s.path.clone();
                        path.push(next);
                        path
                    },
                })
            }
        }
    }

    None
}
