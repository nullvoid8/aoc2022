use std::cmp::max;

use grid::Grid;
use inpt::{inpt, Inpt};
use itertools::Itertools;

type Input = Vec<Direction>;

#[derive(Debug, Inpt, Clone, Copy)]
pub enum Direction {
    #[inpt(regex = "<")]
    L,
    #[inpt(regex = ">")]
    R,
}

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    Ok(inpt(&input).unwrap())
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    // *###
    H,
    //  #
    // ###
    // *#
    C,
    //   #
    //   #
    // *##
    L,
    // #
    // #
    // #
    // *
    V,
    // ##
    // *#
    B,
}

impl Shape {
    fn height(&self) -> usize {
        match self {
            Shape::H => 1,
            Shape::C => 3,
            Shape::L => 3,
            Shape::V => 4,
            Shape::B => 2,
        }
    }

    fn width(&self) -> usize {
        match self {
            Shape::H => 4,
            Shape::C => 3,
            Shape::L => 3,
            Shape::V => 1,
            Shape::B => 2,
        }
    }

    fn cells(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let x = x as usize;
        let y = y as usize;
        match self {
            Shape::H => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Shape::C => vec![
                (x + 1, y),
                (x, y + 1),
                (x + 1, y + 1),
                (x + 2, y + 1),
                (x + 1, y + 2),
            ],
            Shape::L => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            Shape::V => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Shape::B => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
        }
    }

    fn stamp(&self, g: &mut Grid<bool>, x: usize, y: usize) {
        for (x, y) in self.cells(x, y) {
            g[y][x] = true;
        }
    }

    fn is_blocked(&self, g: &Grid<bool>, x: usize, y: usize) -> bool {
        self.cells(x, y).into_iter().any(|(x, y)| g[y][x])
    }
}

pub fn run(input: Input) -> () {
    let mut jets = input.iter().cycle();
    let blocks = [Shape::H, Shape::C, Shape::L, Shape::V, Shape::B]
        .into_iter()
        .cycle();

    let mut g: Grid<bool> = Grid::new(1, 9);
    g.fill(true);
    let mut tallest: usize = 0;

    for block in blocks.take(1000000000000) {
        // require at least tallest + shape.height + 3 rows
        while g.rows() < tallest + block.height() + 4 {
            g.push_row(vec![
                true, false, false, false, false, false, false, false, true,
            ]);
        }

        let mut x: usize = 3;
        let mut y: usize = tallest + 4;

        // let mut h = g.clone();
        // block.stamp(&mut h, x, y);
        // visualise(&h);

        while let Some(&dir) = jets.next() {
            // print!(
            //     "{}",
            //     match dir {
            //         Direction::L => '<',
            //         Direction::R => '>',
            //     }
            // );
            match dir {
                Direction::L => {
                    if !block.is_blocked(&g, x - 1, y) {
                        x -= 1;
                    }
                }
                Direction::R => {
                    if !block.is_blocked(&g, x + 1, y) {
                        x += 1;
                    }
                }
            }

            if block.is_blocked(&g, x, y - 1) {
                break;
            }
            y -= 1;
        }
        // println!();
        // println!();
        block.stamp(&mut g, x, y);
        tallest = max(tallest, y + block.height() - 1);
    }

    println!("{}", tallest);
}

// fn stamp_shape(g: Grid<bool>, (x, y): (usize, usize))

fn visualise(g: &Grid<bool>) {
    let mut rows = (0..g.rows())
        .map(|row| {
            g.iter_row(row)
                .map(|&b| if b { '#' } else { '.' })
                .collect::<String>()
        })
        .rev()
        .collect_vec();

    rows.push("-------".to_owned());

    println!("{}", rows.join("\n"));
}

// fn drop(g: &mut Grid<bool>, block: Shape, x: usize, y: usize) -> usize {}
