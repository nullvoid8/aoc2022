use std::cmp::max;

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

    fn cells(&self, x: usize) -> Vec<u8> {
        match self {
            Shape::H => vec![0b1111 << x],
            Shape::C => vec![0b010 << x, 0b111 << x, 0b010 << x],
            Shape::L => vec![0b111 << x, 0b100 << x, 0b100 << x],
            Shape::V => vec![0b1 << x, 0b1 << x, 0b1 << x, 0b1 << x],
            Shape::B => vec![0b11 << x, 0b11 << x],
        }
    }

    fn stamp(&self, g: &mut Vec<u8>, x: usize, y: usize) {
        let rows = g[y..(y + self.height())].iter_mut();
        let cells = self.cells(x).into_iter();
        rows.zip(cells).for_each(|(x, y)| *x |= y);
    }

    fn is_blocked(&self, g: &Vec<u8>, x: usize, y: usize) -> bool {
        let rows = g[y..(y + self.height())].into_iter();
        let cells = self.cells(x).into_iter();
        rows.zip(cells).any(|(x, y)| (x & y) != 0)
    }

    fn sim_drop<I: Iterator<Item = Direction>>(
        self,
        g: &mut Vec<u8>,
        jets: &mut I,
        mut x: usize,
        mut y: usize,
    ) -> usize {
        while let Some(dir) = jets.next() {
            let mut h = g.clone();
            self.stamp(&mut h, x, y);
            visualise(h);

            match dir {
                Direction::L => {
                    if (x > 0) && !self.is_blocked(&g, x - 1, y) {
                        x -= 1;
                    }
                }
                Direction::R => {
                    if ((x + 1 + self.width()) <= 7) && !self.is_blocked(&g, x + 1, y) {
                        x += 1;
                    }
                }
            }

            if y == 0 || self.is_blocked(&g, x, y - 1) {
                break;
            }
            y -= 1;
        }

        self.stamp(g, x, y);

        y + self.height()
    }
}

pub fn run(input: Input) -> () {
    let mut jets = input.into_iter().cycle();
    let blocks = [Shape::H, Shape::C, Shape::L, Shape::V, Shape::B]
        .into_iter()
        .cycle();

    let mut g: Vec<u8> = Vec::new();
    let mut tallest: usize = 0;

    for block in blocks.take(2) {
        // require at least tallest + shape.height + 3 rows
        while g.len() < tallest + block.height() + 3 {
            g.push(1 << 7);
        }

        tallest = max(tallest, block.sim_drop(&mut g, &mut jets, 2, tallest + 3));
    }

    println!("{tallest}",);

    // visualise(g.clone());
}

// fn stamp_shape(g: Grid<bool>, (x, y): (usize, usize))

fn visualise(mut g: Vec<u8>) {
    g.reverse();

    println!(
        "{}",
        g.into_iter()
            .map(|row| format!("{:>08b}", row.reverse_bits()))
            .join("\n")
            .replace('0', ".")
            .replace('1', "#")
    );
    println!("-------\n");
}
