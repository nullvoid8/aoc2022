use enum_map::{enum_map, Enum, EnumMap};
use itertools::{chain, Itertools};
use std::{
    collections::HashSet,
    iter::{once, FusedIterator},
    mem::replace,
};

use pathfinding::directed::astar;

type Input = Sim;

pub fn parse(sim: String) -> Result<Input, get_inputs::Error> {
    let mut out = Sim {
        width: sim.lines().next().unwrap().len() - 2,
        height: sim.lines().count() - 2,
        blizzards: enum_map! {
                Dir::N => HashSet::default(),
                Dir::S => HashSet::default(),
                Dir::W => HashSet::default(),
                Dir::E => HashSet::default(),
        },
    };

    for (y, line) in sim.lines().skip(1).enumerate() {
        for (x, c) in line.chars().skip(1).enumerate() {
            let p = Point { x, y };
            match c {
                '^' => {
                    out.blizzards[Dir::N].insert(p);
                }
                'v' => {
                    out.blizzards[Dir::S].insert(p);
                }
                '<' => {
                    out.blizzards[Dir::W].insert(p);
                }
                '>' => {
                    out.blizzards[Dir::E].insert(p);
                }
                _ => {}
            }
        }
    }

    Ok(out)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Node {
    Start,
    P(Point),
    End,
}

pub fn run(sim: Input) -> () {
    let there = sim.path(0, Node::Start, Node::End).unwrap();

    println!("{}", there.1);

    let and_back_again = sim.path(there.1, Node::End, Node::Start).unwrap();

    let and_there_again = sim
        .path(there.1 + and_back_again.1, Node::Start, Node::End)
        .unwrap();

    println!("{}", there.1 + and_back_again.1 + and_there_again.1);

    // sim.print_solution(&solution.0);
}

#[derive(Debug, Clone, Copy, Enum, PartialEq, Eq)]
enum Dir {
    N,
    S,
    W,
    E,
}

impl Iterator for Dir {
    type Item = Dir;

    fn next(&mut self) -> Option<Self::Item> {
        Some(replace(
            self,
            match self {
                Dir::N => Dir::S,
                Dir::S => Dir::W,
                Dir::W => Dir::E,
                Dir::E => Dir::N,
            },
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn step(self, d: Dir) -> Point {
        let Point { x, y } = self;
        match d {
            Dir::N => Point { x, y: y - 1 },
            Dir::S => Point { x, y: y + 1 },
            Dir::W => Point { x: x - 1, y },
            Dir::E => Point { x: x + 1, y },
        }
    }
}

#[derive(Debug)]
pub struct Sim {
    width: usize,
    height: usize,
    blizzards: EnumMap<Dir, HashSet<Point>>,
}

impl Sim {
    fn path(&self, t: usize, from: Node, to: Node) -> Option<(Vec<(usize, Node)>, usize)> {
        astar::astar::<(usize, Node), usize, _, _, _, _>(
            &(t, from),
            |&(t, p)| {
                // let t = t + 1;

                self.neighbours(p)
                    .chain(once(p))
                    .filter(move |&p| self.is_passable_at(p, t + 1))
                    .map(move |p| ((t + 1, p), 1))
                    .collect_vec()
            },
            |&(_, p)| match p {
                Node::Start => 1,
                Node::P(_) => 1,
                Node::End => 0,
            },
            |&(_, p)| p == to,
        )
    }

    fn neighbours(&self, p: Node) -> Neighbours {
        Neighbours {
            nw_corner: Point { x: 0, y: 0 },
            se_corner: Point {
                x: self.width - 1,
                y: self.height - 1,
            },
            p,
            d: Dir::N,
            done: false,
        }
    }

    fn time_travel(&self, p: Point, d: Dir, t: usize) -> Point {
        match d {
            Dir::N => Point {
                x: p.x,
                y: (p.y + t) % self.height,
            },
            Dir::S => Point {
                x: p.x,
                y: (p.y + (self.height - (t % self.height))) % self.height,
            },
            Dir::W => Point {
                x: (p.x + t) % self.width,
                y: p.y,
            },
            Dir::E => Point {
                x: (p.x + (self.width - (t % self.width))) % self.width,
                y: p.y,
            },
        }
    }

    fn blizzard_at(&self, p: Point, d: Dir, t: usize) -> bool {
        self.blizzards[d].contains(&self.time_travel(p, d, t))
    }

    fn any_blizzard_at(&self, p: Point, t: usize) -> bool {
        self.blizzards
            .iter()
            .any(|(d, blizzard)| blizzard.contains(&self.time_travel(p, d, t)))
    }

    fn is_passable_at(&self, p: Node, t: usize) -> bool {
        match p {
            Node::Start => true,
            Node::P(p) => !self.any_blizzard_at(p, t),
            Node::End => true,
        }
    }

    #[allow(unused)]
    fn print_solution(&self, path: &Vec<(usize, Node)>) {
        for &(t, p) in path {
            println!("Minute {t}");
            self.print_step(p, t);
            println!();
        }
    }

    fn print_step(&self, player: Node, t: usize) {
        let lines = chain!(
            [format!(
                "#{}{}",
                if player == Node::Start { 'E' } else { '.' },
                "#".repeat(self.width)
            )],
            (0..self.height).map(|y| format!("#{}#", self.print_line(y, player, t))),
            [format!(
                "{}{}#",
                "#".repeat(self.width),
                if player == Node::End { 'E' } else { '.' }
            )],
        )
        .join("\n");

        println!("{}", lines);
    }

    fn print_line(&self, y: usize, player: Node, t: usize) -> String {
        (0..self.width)
            .map(|x| {
                let p = Point { x, y };
                if Node::P(p) == player {
                    return 'E';
                }
                if self.is_passable_at(Node::P(p), t) {
                    return '.';
                }

                let dirs = Dir::N
                    .take(4)
                    .filter(|&d| self.blizzard_at(p, d, t))
                    .map(|d| match d {
                        Dir::N => '^',
                        Dir::S => 'v',
                        Dir::W => '<',
                        Dir::E => '>',
                    })
                    .collect_vec();

                match dirs.len() {
                    0 => {}
                    1 => return dirs[0],
                    x => return x.to_string().chars().nth(0).unwrap(),
                }

                return 'X';
            })
            .collect::<String>()
    }
}

struct Neighbours {
    nw_corner: Point,
    se_corner: Point,
    p: Node,

    d: Dir,
    done: bool,
}

impl Iterator for Neighbours {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        match self.p {
            Node::Start => {
                self.done = true;
                Some(Node::P(self.nw_corner))
            }
            Node::P(p) => match self.d {
                Dir::N => {
                    self.d.next();
                    if p == self.nw_corner {
                        Some(Node::Start)
                    } else if p.y == self.nw_corner.y {
                        self.next()
                    } else {
                        Some(Node::P(p.step(Dir::N)))
                    }
                }
                Dir::S => {
                    self.d.next();

                    if p == self.se_corner {
                        Some(Node::End)
                    } else if p.y == self.se_corner.y {
                        self.next()
                    } else {
                        Some(Node::P(p.step(Dir::S)))
                    }
                }
                Dir::W => {
                    self.d.next();
                    if p.x == self.nw_corner.x {
                        self.next()
                    } else {
                        Some(Node::P(p.step(Dir::W)))
                    }
                }
                Dir::E => {
                    self.done = true;
                    if p.x == self.se_corner.x {
                        self.next()
                    } else {
                        Some(Node::P(p.step(Dir::E)))
                    }
                }
            },

            Node::End => {
                self.done = true;
                Some(Node::P(self.se_corner))
            }
        }
    }
}

impl FusedIterator for Neighbours {}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{Dir, Node, Point};
    use enum_map::enum_map;
    use itertools::Itertools;

    #[test]
    fn neighbours() {
        let sim = super::Sim {
            width: 5,
            height: 5,
            blizzards: enum_map! {
                super::Dir::N => HashSet::new(),
                super::Dir::S => HashSet::new(),
                super::Dir::W => HashSet::new(),
                super::Dir::E => HashSet::new(),
            },
        };

        let cases = [
            (Node::Start, vec![Node::P(Point { x: 0, y: 0 })]),
            (Node::End, vec![Node::P(Point { x: 4, y: 4 })]),
            (
                Node::P(Point { x: 0, y: 0 }),
                vec![
                    Node::Start,
                    Node::P(Point { x: 0, y: 1 }),
                    Node::P(Point { x: 1, y: 0 }),
                ],
            ),
            (
                Node::P(Point { x: 4, y: 0 }),
                vec![Node::P(Point { x: 4, y: 1 }), Node::P(Point { x: 3, y: 0 })],
            ),
            (
                Node::P(Point { x: 0, y: 4 }),
                vec![Node::P(Point { x: 0, y: 3 }), Node::P(Point { x: 1, y: 4 })],
            ),
            (
                Node::P(Point { x: 4, y: 4 }),
                vec![
                    Node::P(Point { x: 4, y: 3 }),
                    Node::End,
                    Node::P(Point { x: 3, y: 4 }),
                ],
            ),
            (
                Node::P(Point { x: 2, y: 2 }),
                vec![
                    Node::P(Point { x: 2, y: 1 }),
                    Node::P(Point { x: 2, y: 3 }),
                    Node::P(Point { x: 1, y: 2 }),
                    Node::P(Point { x: 3, y: 2 }),
                ],
            ),
        ];
        for (p, n) in cases {
            assert_eq!(sim.neighbours(p).collect_vec(), n);
        }
    }

    #[test]
    fn blizzards() {
        let sim = super::Sim {
            width: 5,
            height: 5,
            blizzards: enum_map! {
                super::Dir::N => HashSet::new(),
                super::Dir::S => HashSet::from_iter([Point{x:0,y:0}]),
                super::Dir::W => HashSet::new(),
                super::Dir::E => HashSet::new(),
            },
        };

        let cases = [
            (Dir::S, Point { x: 0, y: 0 }, 0),
            (Dir::S, Point { x: 0, y: 1 }, 1),
            (Dir::S, Point { x: 0, y: 2 }, 2),
            (Dir::S, Point { x: 0, y: 3 }, 3),
            (Dir::S, Point { x: 0, y: 4 }, 4),
        ];

        for (d, p, t) in cases {
            assert!(sim.blizzard_at(p, d, t));
            assert!(sim.any_blizzard_at(p, t));
        }
    }
}
