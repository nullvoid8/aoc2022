use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fmt::Display,
    mem::replace,
};

use itertools::Itertools;

type Input = HashSet<Point>;

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    Ok(input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                (c == '#').then_some(Point {
                    x: x as i64,
                    y: y as i64,
                })
            })
        })
        .collect())
}

pub fn run(init_elves: Input) -> () {
    let mut sim = Sim::new(init_elves);
    let p1 = sim.nth(10).unwrap().free_space();
    println!("{p1}");
    let p2 = sim.take_while(|x| !x.stopped).count() + 10 + 1;
    println!("{p2}");
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    S,
    W,
    E,
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dir::N => f.write_str("N"),
            Dir::S => f.write_str("S"),
            Dir::W => f.write_str("W"),
            Dir::E => f.write_str("E"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn step(&self, dir: Dir) -> Self {
        match self {
            &Point { x, y } => match dir {
                Dir::N => Self { x, y: y - 1 },
                Dir::S => Self { x, y: y + 1 },
                Dir::W => Self { x: x - 1, y },
                Dir::E => Self { x: x + 1, y },
            },
        }
    }

    fn neighbours(&self, dir: Dir) -> [Point; 3] {
        use Dir::*;
        let p = self.step(dir);
        match dir {
            Dir::N => [p.step(W), p, p.step(E)],
            Dir::S => [p.step(W), p, p.step(E)],
            Dir::W => [p.step(N), p, p.step(S)],
            Dir::E => [p.step(N), p, p.step(S)],
        }
    }

    fn can_move(&self, others: &HashSet<Self>, dir: Dir) -> bool {
        !self.neighbours(dir).iter().any(|n| others.contains(n))
    }
}

struct Sim {
    dirs: [Dir; 4],
    positions: HashSet<Point>,
    stopped: bool,
}

impl Sim {
    fn new(init: HashSet<Point>) -> Self {
        Sim {
            dirs: [Dir::N, Dir::S, Dir::W, Dir::E],
            positions: init,
            stopped: false,
        }
    }
}

impl Iterator for Sim {
    type Item = ElfMap;

    fn next(&mut self) -> Option<Self::Item> {
        // {dest: start}
        let mut proposed = HashMap::new();
        // conflicted destinations
        let mut conflict = HashSet::new();
        // elves that are sufficently spaced
        let mut unmoved = HashSet::new();

        for &elf in &self.positions {
            if self
                .dirs
                .iter()
                .all(|&dir| elf.can_move(&self.positions, dir))
            {
                unmoved.insert(elf);
                continue;
            }

            let dir = self
                .dirs
                .into_iter()
                .find(|&dir| elf.can_move(&self.positions, dir));

            match dir {
                Some(dir) => {
                    let dest = elf.step(dir);

                    if conflict.contains(&dest) {
                        //  Known conflict, don't move
                        proposed.insert(elf, elf);
                    } else if let Some(other) = proposed.remove(&dest) {
                        //  Discovered conflict, don't move and reset conflicting elf
                        proposed.extend([(elf, elf), (other, other)]);
                        conflict.insert(dest);
                    } else {
                        // No conflict
                        proposed.insert(dest, elf);
                    }
                }
                None => {
                    proposed.insert(elf, elf);
                }
            }
        }

        let mut next = unmoved;
        next.extend(proposed.keys());

        self.dirs.rotate_left(1);

        Some(ElfMap {
            map: replace(&mut self.positions, next),
            stopped: replace(&mut self.stopped, proposed.is_empty()),
        })
    }
}

struct ElfMap {
    map: HashSet<Point>,
    stopped: bool,
}

impl ElfMap {
    fn bounds(&self) -> (Point, Point) {
        match (
            self.map.iter().copied().reduce(|acc, p| Point {
                x: min(acc.x, p.x),
                y: min(acc.y, p.y),
            }),
            self.map.iter().copied().reduce(|acc, p| Point {
                x: max(acc.x, p.x),
                y: max(acc.y, p.y),
            }),
        ) {
            (None, None) => (Point::default(), Point::default()),
            (Some(min), Some(max)) => (min, max),
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let (min, max) = self.bounds();

        let out: String = (min.y..=max.y)
            .map(|y| {
                (min.x..=max.x)
                    .map(|x| {
                        if self.map.contains(&Point { x, y }) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .join("\n");

        println!("{out}")
    }

    fn free_space(&self) -> usize {
        let (min, max) = self.bounds();
        let total = ((max.x.abs_diff(min.x) + 1) * (max.y.abs_diff(min.y) + 1)) as usize;
        total - self.map.len()
    }
}
