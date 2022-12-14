use grid::Grid;
use itertools::Itertools;
use pathfinding::directed::astar;

type Point = (usize, usize);

#[derive(Debug)]
pub struct Input {
    start: Point,
    goal: Point,
    grid: Grid<usize>,
}

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    let mut out = Input {
        start: (0, 0),
        goal: (0, 0),
        grid: Grid::new(0, 0),
    };

    input
        .lines()
        .enumerate()
        .map(|(longitude, line)| {
            line.chars()
                .enumerate()
                .map(|(latitude, hill)| match hill {
                    'S' => {
                        out.start = (longitude, latitude);
                        0
                    }
                    'E' => {
                        out.goal = (longitude, latitude);
                        25
                    }
                    hill => hill as usize - 'a' as usize,
                })
                .collect_vec()
        })
        .for_each(|latitude| out.grid.push_row(latitude));

    Ok(out)
}

pub fn run(Input { start, goal, grid }: Input) -> () {
    let (p1, _) = astar::astar(
        &start,
        |center| {
            let mut possibles = Vec::with_capacity(4);
            if center.0 > 0 {
                possibles.push((center.0 - 1, center.1));
            }
            if center.1 > 0 {
                possibles.push((center.0, center.1 - 1));
            }

            possibles.push((center.0 + 1, center.1));
            possibles.push((center.0, center.1 + 1));

            possibles
                .into_iter()
                .filter_map(|n| {
                    let &here = grid.get(center.0, center.1)?;
                    let &there = grid.get(n.0, n.1)?;

                    if here + 1 < there {
                        return None;
                    }
                    Some((n, here.abs_diff(there) + 1))
                })
                .collect_vec()
        },
        |tail| (goal.0.abs_diff(tail.0)) + (goal.1.abs_diff(tail.1)),
        |&tail| tail == goal,
    )
    .unwrap();

    println!("{}", p1.len() - 1);

    let (p2, _) = astar::astar(
        &goal,
        |center| {
            let mut possibles = Vec::with_capacity(4);
            if center.0 > 0 {
                possibles.push((center.0 - 1, center.1));
            }
            if center.1 > 0 {
                possibles.push((center.0, center.1 - 1));
            }

            possibles.push((center.0 + 1, center.1));
            possibles.push((center.0, center.1 + 1));

            possibles
                .into_iter()
                .filter_map(|n| {
                    let &here = grid.get(center.0, center.1)?;
                    let &there = grid.get(n.0, n.1)?;

                    // println!("{} - {} = {}", here, there, here <= there + 1);

                    if there + 1 < here {
                        return None;
                    }
                    Some((n, here.abs_diff(there) + 1))
                })
                .collect_vec()
        },
        |_| 0,
        |n| {
            if let Some(&h) = grid.get(n.0, n.1) {
                return h == 0;
            }
            false
        },
    )
    .unwrap();
    println!("{}", p2.len() - 1);
}

// 5+1 < 4 continue
// 5+1 < 5 continue
// 5+1 < 6 continue
// 5+1 < 7 skip
