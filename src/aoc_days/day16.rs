use std::{cmp::min, collections::HashMap};

use grid::Grid;
use itertools::Itertools;
use nom::IResult;
type Input = Graph;

#[derive(Debug)]
pub struct Graph {
    start: usize,
    flows: Vec<i32>,
    costs: Grid<i32>,
}

#[derive(Debug)]
pub struct RoomDesc {
    label: String,
    flow_rate: i32,
    connections: Vec<String>,
}

fn parse_room(i: &str) -> IResult<&str, RoomDesc> {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        character::complete::i32,
        combinator::map,
        multi::separated_list1,
        sequence::tuple,
    };

    map(
        tuple((
            tag::<_, &str, _>("Valve "),
            map(take(2 as usize), |s: &str| s.to_string()),
            tag(" has flow rate="),
            i32,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), map(take(2 as usize), |s: &str| s.to_string())),
        )),
        |(_, label, _, flow_rate, _, connections)| RoomDesc {
            label,
            flow_rate,
            connections,
        },
    )(i)
}

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    use nom::{character::complete::line_ending, multi::separated_list1};

    let rooms = separated_list1(line_ending, parse_room)(&input).unwrap().1;

    let label_idx: HashMap<_, _> = rooms
        .iter()
        .enumerate()
        .map(|(i, r)| (r.label.clone(), i))
        .collect();

    let mut wg: Grid<i32> = Grid::init(rooms.len(), rooms.len(), (rooms.len() as i32) * 2);

    for src in &rooms {
        for dst in &src.connections {
            wg[label_idx[&src.label]][label_idx[dst]] = 1;
        }
    }

    for i in 0..rooms.len() {
        wg[i][i] = 0;
    }

    for k in 0..rooms.len() {
        for i in 0..rooms.len() {
            for j in 0..rooms.len() {
                match (wg[i][k], wg[k][j]) {
                    (x, y) => wg[i][j] = min(wg[i][j], x + y),
                }
            }
        }
    }

    Ok(Graph {
        start: rooms.iter().find_position(|r| (r.label == "AA")).unwrap().0,
        flows: rooms.iter().map(|r| r.flow_rate).collect(),
        costs: wg,
    })
}

pub fn run(g: Input) -> () {
    // all valves with flow
    let rooms = g
        .flows
        .iter()
        .enumerate()
        .filter_map(|(i, &fr)| (fr != 0).then_some(i))
        .collect_vec();

    // let p1 = dfs_single(&g, g.start, rooms.clone(), 30);
    let p1_worker = Worker {
        to: g.start,
        time: 30,
    };
    let p1 = dfs_single(&g, rooms.clone(), p1_worker);

    println!("{}", p1);

    let p2_worker = Worker {
        to: g.start,
        time: 26,
    };
    let p2 = double_runner(&g, rooms, p2_worker, p2_worker);
    println!("{}", p2);
}

fn dfs_single(g: &Graph, available: Vec<usize>, Worker { to: here, time }: Worker) -> i32 {
    let pressure = g.flows[here] * time;
    let mut pressures = Vec::with_capacity(available.len());

    for &next in &available {
        if g.costs[here][next] > time {
            continue;
        };
        let worker = Worker {
            to: next,
            time: time - g.costs[here][next] - 1,
        };

        let available = available
            .iter()
            .copied()
            .filter(|&x| x != next)
            .collect_vec();

        pressures.push(dfs_single(g, available, worker));
    }

    pressure + pressures.into_iter().max().unwrap_or(0)
}

#[derive(Debug, Clone, Copy)]
struct Worker {
    to: usize,
    time: i32,
}

fn double_runner(
    g: &Graph,
    available: Vec<usize>,
    Worker { to: here, time }: Worker,
    worker_2: Worker,
) -> i32 {
    let pressure = g.flows[here] * time;

    let mut pressures = Vec::with_capacity(available.len());

    for &next in &available {
        if g.costs[here][next] >= time {
            continue;
        };
        let worker_1 = Worker {
            to: next,
            time: time - g.costs[here][next] - 1,
        };

        let available = available
            .iter()
            .copied()
            .filter(|&x| x != next)
            .collect_vec();

        pressures.push(if worker_1.time > worker_2.time {
            double_runner(g, available, worker_1, worker_2)
        } else {
            double_runner(g, available, worker_2, worker_1)
        })
    }

    pressures.push(dfs_single(g, available, worker_2));

    pressure + pressures.into_iter().max().unwrap_or(0)
}
