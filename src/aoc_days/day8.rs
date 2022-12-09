use std::cmp;

use grid::Grid;
use itertools::Itertools;

#[derive(Debug, Default)]
pub struct Tree {
    height: i32,
    visible: bool,
    score: i32,
}

type Input = Grid<Tree>;

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    let mut g = Grid::new(0, 0);

    for line in input.lines() {
        g.push_row(
            line.chars()
                .filter_map(|c| {
                    Some(Tree {
                        height: c.to_digit(10)?.try_into().ok()?,
                        visible: false,
                        score: 1,
                    })
                })
                .collect_vec(),
        );
    }

    let (n_rows, n_cols) = g.size();

    for row in 0..n_rows {
        println!("row {} forward", row);
        handle_trees(g.iter_row_mut(row));
        println!("row {} backward", row);
        handle_trees(g.iter_row_mut(row).rev());
    }

    for col in 0..n_cols {
        println!("col {} forward", col);
        handle_trees(g.iter_col_mut(col));
        println!("col {} backward", col);
        handle_trees(g.iter_col_mut(col).rev());
    }

    Ok(g)
}

fn handle_trees<'a, I: Iterator<Item = &'a mut Tree>>(iter: I) {
    let mut tallest = -1;
    let mut dist = [0; 10];

    for t in iter {
        t.visible |= tallest < t.height;
        t.score *= dist[t.height as usize];

        for j in 0..10 {
            if j <= (t.height as usize) {
                dist[j] = 1;
            } else {
                dist[j] += 1;
            }
        }
        tallest = cmp::max(tallest, t.height);
    }
}

pub fn run(g: Input) -> () {
    let visible = g.iter().filter(|t| t.visible).count();
    let best = g.iter().map(|t| t.score).max().unwrap_or_default();

    println!("{}", visible);
    println!("{}", best);
}
