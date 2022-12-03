use std::{
    fmt::{self},
    fs,
};

use itertools::{iproduct, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::i32,
    combinator::map,
    sequence::{delimited, separated_pair},
    IResult,
};
use num_integer::{div_ceil, div_floor};

#[derive(Debug, PartialEq, Eq)]
enum Instr {
    None,
    Reduced,
    Explode(i32, i32),
    ExplodeL(i32),
    ExplodeR(i32),
}

#[derive(Clone)]
enum SFNum {
    Lit(i32),
    Nest(Box<SFNum>, Box<SFNum>),
}

impl fmt::Debug for SFNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lit(arg0) => f.write_fmt(format_args!("{}", arg0)),
            Self::Nest(arg0, arg1) => f.write_fmt(format_args!("[{:?},{:?}]", arg0, arg1)),
        }
    }
}

fn add_to_rightmost(root: &mut SFNum, x: i32) {
    match root {
        SFNum::Lit(a) => *a += x,
        SFNum::Nest(_, node) => add_to_rightmost(node, x),
    }
}

fn add_to_leftmost(root: &mut SFNum, x: i32) {
    match root {
        SFNum::Lit(a) => *a += x,
        SFNum::Nest(node, _) => add_to_leftmost(node, x),
    }
}

fn explode(root: &mut SFNum) -> bool {
    fn go(node: &mut SFNum, depth: usize) -> Instr {
        match node {
            SFNum::Lit(_) => Instr::None,
            SFNum::Nest(left, right) if depth < 4 => {
                match (left.clone(), go(left, depth + 1)).1 {
                    Instr::None => {}
                    Instr::Explode(a, b) => {
                        *left = SFNum::Lit(0).into();
                        add_to_leftmost(right, b);
                        return Instr::ExplodeL(a);
                    }
                    Instr::ExplodeR(x) => {
                        add_to_leftmost(right, x);
                        return Instr::Reduced;
                    }
                    instr => return instr,
                };

                match (right.clone(), go(right, depth + 1)).1 {
                    Instr::Explode(a, b) => {
                        *right = SFNum::Lit(0).into();
                        add_to_rightmost(left, a);
                        return Instr::ExplodeR(b);
                    }
                    Instr::ExplodeL(x) => {
                        add_to_rightmost(left, x);
                        return Instr::Reduced;
                    }
                    instr => return instr,
                }
            }
            SFNum::Nest(left, right) => match (left.as_ref(), right.as_ref()) {
                (SFNum::Lit(a), SFNum::Lit(b)) => Instr::Explode(*a, *b),
                _ => panic!("invalid SFNum"),
            },
        }
    }
    go(root, 0) != Instr::None
}

fn split(root: &mut SFNum) -> bool {
    match root {
        SFNum::Lit(x) if *x < 10 => false,
        SFNum::Lit(x) => {
            *root = SFNum::Nest(
                SFNum::Lit(div_floor(*x, 2)).into(),
                SFNum::Lit(div_ceil(*x, 2)).into(),
            );
            true
        }
        SFNum::Nest(left, right) => split(left) || split(right),
    }
}

fn reduce(root: &mut SFNum) {
    while explode(root) || split(root) {}
}

fn add_sfnum<'a>(a: SFNum, b: SFNum) -> SFNum {
    let mut out = SFNum::Nest(a.into(), b.into());
    reduce(&mut out);
    out
}

fn parse_lit(i: &str) -> IResult<&str, SFNum> {
    map(i32, |x| SFNum::Lit(x))(i)
}

fn parse_pair(i: &str) -> IResult<&str, SFNum> {
    map(
        delimited(
            tag("["),
            separated_pair(parse_snailfish, tag(","), parse_snailfish),
            tag("]"),
        ),
        |(a, b)| SFNum::Nest(a.into(), b.into()),
    )(i)
}

fn parse_snailfish(i: &str) -> IResult<&str, SFNum> {
    alt((parse_lit, parse_pair))(i)
}

fn calc_magnitude(root: SFNum) -> i32 {
    match root {
        SFNum::Lit(x) => x,
        SFNum::Nest(a, b) => 3 * calc_magnitude(*a) + 2 * calc_magnitude(*b),
    }
}

pub fn day() {
    let content = fs::read_to_string("inputs/day18").expect("Couldn't find input");
    let nums = content
        .lines()
        .map(parse_snailfish)
        .map(|x| x.unwrap().1)
        .collect_vec();

    let mut p1 = nums[0].clone();
    for sfn in nums.iter().skip(1) {
        p1 = add_sfnum(p1, sfn.clone());
    }

    let p1 = calc_magnitude(p1);

    let p2 = iproduct!(nums.clone(), nums.clone())
        .map(|(a, b)| calc_magnitude(add_sfnum(a, b)))
        .max()
        .unwrap();

    print!("{} {}", p1, p2);
}
