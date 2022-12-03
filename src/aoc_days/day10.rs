use cgmath::{prelude::*, Vector4};
use std::fs;

use itertools::Itertools;
use phf::phf_map;

struct PartialParse {
    before: Vec<char>,
    after: Vec<char>,
}

static PAIRS: phf::Map<char, char> = phf_map! {
  '(' => ')',
  '[' => ']',
  '{' => '}',
  '<' => '>',
  ')' => '(',
  ']' => '[',
  '}' => '{',
  '>' => '<',
};

fn parse_parens(i: &str) -> PartialParse {
    let mut out: PartialParse = PartialParse {
        before: Vec::new(),
        after: Vec::new(),
    };

    for c in i.chars() {
        match c {
            '(' | '{' | '[' | '<' => {
                out.after.push(c);
            }
            '>' | ']' | '}' | ')' => {
                if out.after.last() == Some(&PAIRS[&c]) {
                    out.after.pop();
                } else {
                    out.before.push(c);
                }
            }
            _ => {}
        }
    }

    out
}

fn paren_digit(c: char) -> char {
    match c {
        '(' | ')' => '1',
        '[' | ']' => '2',
        '{' | '}' => '3',
        '<' | '>' => '4',
        _ => '0',
    }
}

pub fn day() {
    let content = fs::read_to_string("inputs/day10").expect("Couldn't find input");
    let partials = content.lines().map(parse_parens).collect_vec();

    let scores = Vector4::new(3, 57, 1197, 25137);

    let (incompleted, corrupted): (Vec<_>, _) =
        partials.into_iter().partition(|pp| pp.before.is_empty());

    let p1 = scores.dot(
        corrupted
            .into_iter()
            .map(|parens| match parens.before.first() {
                Some(')') => Vector4::new(1, 0, 0, 0),
                Some(']') => Vector4::new(0, 1, 0, 0),
                Some('}') => Vector4::new(0, 0, 1, 0),
                Some('>') => Vector4::new(0, 0, 0, 1),
                _ => Vector4::new(0, 0, 0, 0),
            })
            .sum(),
    );

    let mut p2 = incompleted
        .into_iter()
        .map(|pp| {
            pp.after
                .into_iter()
                .rev()
                .map(|c| paren_digit(c))
                .collect::<String>()
        })
        .map(|src| u64::from_str_radix(&src, 5).unwrap())
        .collect_vec();
    p2.sort();

    let p2 = p2[p2.len() / 2];

    println!("{} {}", p1, p2);
}
