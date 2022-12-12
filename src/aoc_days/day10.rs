use inpt::{inpt, Inpt};

type Input = Vec<Command>;

#[derive(Debug, Inpt)]
pub enum Command {
    #[inpt(regex = r"noop")]
    NOOP,
    #[inpt(regex = r"addx (-?\d+)")]
    ADDX { v: i64 },
}

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    Ok(inpt(&input).unwrap())
}

pub fn run(input: Input) -> () {
    let program = input
        .into_iter()
        .flat_map(|x| match x {
            Command::NOOP => vec![Command::NOOP],
            Command::ADDX { v } => vec![Command::NOOP, Command::ADDX { v }],
        })
        .enumerate();

    let mut p1 = 0;
    let mut p2: String = String::default();

    let mut reg_x = 1;

    for (i, cmd) in program {
        if i % 40 == 19 {
            p1 += (i as i64 + 1) * reg_x;
        }

        p2 += if reg_x.abs_diff(i as i64 % 40) <= 1 {
            "#"
        } else {
            " "
        };
        if i % 40 == 39 {
            p2 += "\n"
        }
        if let Command::ADDX { v } = cmd {
            reg_x += v
        }
    }

    println!("{}", p1);
    println!("{}", p2);
}
