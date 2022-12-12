// This file does not get checked by rust as it does not participate in the module tree

use std::collections::VecDeque;

use inpt::{inpt, Inpt};
use itertools::Itertools;

#[derive(Debug, Inpt, Clone)]
pub enum Operation {
    #[inpt(regex = r"new = old \+ (\d+)")]
    Add(usize),
    #[inpt(regex = r"new = old \* (\d+)")]
    Mul(usize),
    #[inpt(regex = r"new = old \* old")]
    Square,
}

impl Operation {
    fn apply(&self, x: usize) -> usize {
        match self {
            Operation::Add(y) => x + y,
            Operation::Mul(y) => x * y,
            Operation::Square => x * x,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: usize,
    // true, false
    target: (usize, usize),
}

type Input = Vec<Monkey>;

pub fn parse(monkies: String) -> Result<Input, get_inputs::Error> {
    Ok(monkies
        .lines()
        .chunks(7)
        .into_iter()
        .map(|chunk| -> Monkey {
            match chunk.collect_vec().as_slice() {
                &[_, items, operation, test, if_true, if_false, ..] => Monkey {
                    items: inpt::<Vec<_>>(items.strip_prefix("  Starting items: ").unwrap())
                        .unwrap()
                        .into(),
                    operation: inpt(operation.strip_prefix("  Operation: ").unwrap()).unwrap(),
                    test: inpt(test.strip_prefix("  Test: divisible by").unwrap()).unwrap(),
                    target: (
                        inpt(
                            if_true
                                .strip_prefix("    If true: throw to monkey")
                                .unwrap(),
                        )
                        .unwrap(),
                        inpt(
                            if_false
                                .strip_prefix("    If false: throw to monkey")
                                .unwrap(),
                        )
                        .unwrap(),
                    ),
                },
                _ => panic!("small monkey"),
            }
        })
        .collect_vec())
}

pub fn run(input: Input) -> () {
    println!("{}", run_p1(input.clone()));
    println!("{}", run_p2(input.clone()));
}

fn run_p1(mut monkies: Input) -> usize {
    let mut inspections = vec![0; monkies.len()];

    for _ in 0..20 {
        for i in 0..monkies.len() {
            inspections[i] += monkies[i].items.len();
            while let Some(worry) = monkies[i].items.pop_back() {
                let worry = monkies[i].operation.apply(worry) / 3;
                let send_to = if worry % monkies[i].test == 0 {
                    monkies[i].target.0
                } else {
                    monkies[i].target.1
                };
                monkies[send_to].items.push_back(worry);
            }
        }
    }

    inspections.sort();
    inspections.reverse();
    inspections.into_iter().take(2).product()
}

fn run_p2(mut monkies: Input) -> usize {
    let mut inspections = vec![0; monkies.len()];

    let cm: usize = monkies.iter().map(|m| m.test).product();

    for _ in 0..10000 {
        for i in 0..monkies.len() {
            inspections[i] += monkies[i].items.len();
            while let Some(worry) = monkies[i].items.pop_back() {
                let worry = monkies[i].operation.apply(worry) % cm;
                let send_to = if worry % monkies[i].test == 0 {
                    monkies[i].target.0
                } else {
                    monkies[i].target.1
                };
                monkies[send_to].items.push_back(worry);
            }
        }
    }

    inspections.sort();
    inspections.reverse();
    inspections.into_iter().take(2).product()
}
