use inpt::{inpt, Inpt};
use regex::Regex;

type Input = (Vec<Stack>, Vec<Move>);

pub type Stack = Vec<char>;

fn parse_stacks(s: &str) -> Vec<Stack> {
    let mut stacks: Vec<Stack> = vec![
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    let stack_re = Regex::new(r"\[.\]").unwrap();

    s.lines().for_each(|line| {
        stack_re.find_iter(line).for_each(|package| {
            stacks
                .get_mut(package.start() / 4)
                .unwrap()
                .push(package.as_str().chars().nth(1).unwrap());
        });
    });

    for stack in &mut stacks {
        stack.reverse();
    }

    stacks
}

#[derive(Debug, Inpt)]
#[inpt(regex = r"move (\d+) from (\d+) to (\d+)")]
pub struct Move {
    count: usize,
    from: usize,
    to: usize,
}

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    Ok((
        parse_stacks(stacks),
        moves
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| match inpt(s).unwrap() {
                Move { count, from, to } => Move {
                    count,
                    from: from - 1,
                    to: to - 1,
                },
            })
            .collect(),
    ))
}

pub fn run((stacks, moves): Input) -> () {
    let mut mover9000 = stacks.clone();

    for &Move { count, from, to } in &moves {
        let mut moved = {
            let stack = mover9000.get_mut(from).unwrap();
            stack.split_off(stack.len() - count)
        };
        moved.reverse();

        mover9000.get_mut(to).unwrap().extend(moved);
    }

    let top9000 = mover9000
        .iter()
        .map(|stack| match stack.last() {
            None => "".to_owned(),
            Some(c) => c.to_string(),
        })
        .collect::<String>();

    println!("{}", top9000);

    let mut mover9001 = stacks.clone();

    for Move { count, from, to } in moves {
        let moved = {
            let stack = mover9001.get_mut(from).unwrap();
            stack.split_off(stack.len() - count)
        };

        mover9001.get_mut(to).unwrap().extend(moved);
    }

    let top9001 = mover9001
        .iter()
        .map(|stack| match stack.last() {
            None => "".to_owned(),
            Some(c) => c.to_string(),
        })
        .collect::<String>();

    println!("{}", top9001);
}
