use enum_map::{enum_map, Enum, EnumMap};
use itertools::Itertools;
use std::{collections::HashSet, iter::once};

use pathfinding::directed::astar;

type Input = Sim;

pub fn parse(sim: String) -> Result<Input, get_inputs::Error> {
    let width = sim.lines().next().unwrap().len() as isize;
    let height = sim.lines().count() as isize;
    let mut out = Sim {
        width,
        height,
        blizzards: Blizzards {
            width: width - 2,
            height: height - 2,
            blizzards: enum_map! {
                Dir::N => HashSet::default(),
                Dir::S => HashSet::default(),
                Dir::W => HashSet::default(),
                Dir::E => HashSet::default(),
            },
        },
    };

    for (y, line) in sim.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = Point {
                x: x as isize - 1,
                y: y as isize - 1,
            };
            match c {
                '^' => {
                    out.blizzards.blizzards[Dir::N].insert(p);
                }
                'v' => {
                    out.blizzards.blizzards[Dir::S].insert(p);
                }
                '<' => {
                    out.blizzards.blizzards[Dir::W].insert(p);
                }
                '>' => {
                    out.blizzards.blizzards[Dir::E].insert(p);
                }
                _ => {}
            }
        }
    }

    Ok(out)
}

pub fn run(sim: Input) -> () {
    let solution = astar::astar::<(isize, Point), usize, _, _, _, _>(
        &(0, sim.init()),
        |&(t, p)| {
            let sim = &sim;
            // let t = t + 1;

            once(p)
                .chain(
                    p.neighbours()
                        .into_iter()
                        .filter_map(move |p| (sim).is_passable_at(p, t).then_some(p)),
                )
                .map(move |p| ((t + 1, p), 1))
        },
        |&(_, p)| {
            let d = sim.dest();
            p.x.abs_diff(d.x) + p.y.abs_diff(d.y)
        },
        |&(_, p)| p == sim.dest(),
    )
    .unwrap();

    // println!("{}", solution.1);
    sim.print_solution(&solution.0);
}

#[derive(Debug, Clone, Copy, Enum, PartialEq, Eq)]
enum Dir {
    N,
    S,
    W,
    E,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn step_by(self, t: isize, d: Dir) -> Point {
        let Point { x, y } = self;
        match d {
            Dir::N => Point { x, y: y - t },
            Dir::S => Point { x, y: y + t },
            Dir::W => Point { x: x - t, y },
            Dir::E => Point { x: x + t, y },
        }
    }

    fn neighbours(&self) -> [Point; 4] {
        [
            self.step_by(1, Dir::N),
            self.step_by(1, Dir::S),
            self.step_by(1, Dir::W),
            self.step_by(1, Dir::E),
        ]
    }
}

// type Blizzards = HashSet<Point>;

#[derive(Debug)]
struct Blizzards {
    width: isize,
    height: isize,
    blizzards: EnumMap<Dir, HashSet<Point>>,
}

impl Blizzards {
    fn wrap_point(&self, p: Point) -> Point {
        Point {
            x: p.x.rem_euclid(self.width),
            y: p.y.rem_euclid(self.height),
        }
    }

    fn contains_at(&self, d: Dir, p: Point, t: isize) -> bool {
        let p = Point {
            x: p.x - 1,
            y: p.y - 1,
        };
        self.blizzards[d].contains(&self.wrap_point(p.step_by(-t, d)))
    }

    fn contains_any_at(&self, p: Point, t: isize) -> bool {
        let p = Point {
            x: p.x - 1,
            y: p.y - 1,
        };
        self.blizzards
            .iter()
            .any(|(d, bliz)| bliz.contains(&self.wrap_point(p.step_by(-t, d))))
    }
}

#[derive(Debug)]
pub struct Sim {
    width: isize,
    height: isize,
    blizzards: Blizzards,
}

impl Sim {
    fn init(&self) -> Point {
        Point { x: 1, y: 0 }
    }
    fn dest(&self) -> Point {
        Point {
            x: self.width - 2,
            y: self.height - 1,
        }
    }

    fn is_wall(&self, p: Point) -> bool {
        // height = width = 4
        //   0123
        // 0 #i##
        // 1 #..#
        // 2 #..#
        // 3 ##d#

        if p == self.init() {
            return false;
        }
        if p == self.dest() {
            return false;
        }

        if p.x <= 0 || p.y <= 0 {
            return true;
        }

        if p.x >= self.width - 1 || p.y >= self.height - 1 {
            return true;
        }

        return false;
    }

    fn is_passable_at(&self, p: Point, t: isize) -> bool {
        p == self.init()
            || p == self.dest()
            || (!self.is_wall(p) && !self.blizzards.contains_any_at(p, t))
    }

    #[allow(unused)]
    fn print_solution(&self, path: &Vec<(isize, Point)>) {
        for &(t, p) in path {
            println!("Minute {t}");
            self.print_step(p, t);
            println!();
        }
    }

    #[allow(unused)]
    fn print_step(&self, player: Point, t: isize) {
        let lines = (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(move |x| {
                        let p = Point { x, y };
                        // if p == player {
                        //     return 'E';
                        // }

                        if self.is_passable_at(p, t) {
                            return '.';
                        }

                        if self.is_wall(p) {
                            return '#';
                        }

                        let dirs = [Dir::N, Dir::S, Dir::W, Dir::E]
                            .into_iter()
                            .filter(|&d| self.blizzards.contains_at(d, p, t))
                            .collect_vec();
                        match dirs.as_slice() {
                            [Dir::N] => return '^',
                            [Dir::S] => return 'v',
                            [Dir::W] => return '<',
                            [Dir::E] => return '>',
                            [_, _] => return '2',
                            [_, _, _] => return '3',
                            [_, _, _, _] => return '4',
                            _ => {}
                        };

                        return 'X';
                    })
                    .collect::<String>()
            })
            .join("\n");

        println!("{}", lines);
    }
}
