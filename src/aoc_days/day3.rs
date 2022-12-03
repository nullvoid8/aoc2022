use itertools::Itertools;
use std::{cmp::Ordering, fs};

fn read_lines(day: i32) -> Vec<String> {
    fs::read_to_string(format!("inputs/day{}", day))
        .unwrap()
        .lines()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
}

fn bit_counts(xs: &Vec<Vec<bool>>) -> Vec<Ordering> {
    let mut counts = std::iter::repeat(0).take(xs[0].len()).collect_vec();
    let len = xs.len() as i32;
    xs.iter().for_each(|xs| {
        xs.iter()
            .enumerate()
            .for_each(|(n, b)| counts[n] += *b as i32)
    });
    counts.iter().map(|c| (2 * c).cmp(&len)).collect_vec()
}

#[derive(Clone, Copy)]
struct Foo(i32);

impl FromIterator<bool> for Foo {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Foo {
        let mut out = 0;
        iter.into_iter().for_each(|b| out = (2 * out) + (b as i32));
        Foo(out)
    }
}

impl<'a> FromIterator<&'a bool> for Foo {
    fn from_iter<T: IntoIterator<Item = &'a bool>>(iter: T) -> Foo {
        let mut out = 0;
        iter.into_iter().for_each(|b| out = (2 * out) + (*b as i32));
        Foo(out)
    }
}

impl From<Foo> for i32 {
    fn from(Foo(x): Foo) -> i32 {
        x
    }
}

pub fn day() {
    // let mut counts = [0, 0, 0, 0, 0];

    let lines = read_lines(3);
    let lines = lines
        .iter()
        .map(|x| {
            x.chars()
                .map(|c| match c {
                    '0' => false,
                    '1' => true,
                    _ => panic!("Bad Char"),
                })
                .collect_vec()
        })
        .collect_vec();
    let len = lines[0].len();

    let counts = bit_counts(&lines);

    let gamma: i32 = counts.iter().map(|x| x.is_gt()).collect::<Foo>().into();
    let epsilon: i32 = counts.iter().map(|x| x.is_lt()).collect::<Foo>().into();

    let mut oxy = lines.clone();
    let mut co2 = lines.clone();

    for n in 0..len as usize {
        let counts = bit_counts(&oxy);
        oxy = match counts[n] {
            Ordering::Less => oxy.into_iter().filter(|xs| !xs[n]).collect_vec(),
            Ordering::Equal => oxy.into_iter().filter(|xs| xs[n]).collect_vec(),
            Ordering::Greater => oxy.into_iter().filter(|xs| xs[n]).collect_vec(),
        };
        if oxy.len() == 1 {
            break;
        }
    }

    for n in 0..len as usize {
        let counts = bit_counts(&co2);
        co2 = match counts[n] {
            Ordering::Less => co2.into_iter().filter(|xs| xs[n]).collect_vec(),
            Ordering::Equal => co2.into_iter().filter(|xs| !xs[n]).collect_vec(),
            Ordering::Greater => co2.into_iter().filter(|xs| !xs[n]).collect_vec(),
        };
        if co2.len() == 1 {
            break;
        }
    }

    let oxy: i32 = oxy[0].iter().collect::<Foo>().into();
    let co2: i32 = co2[0].iter().collect::<Foo>().into();

    println!("{} {}", gamma * epsilon, oxy * co2);
}
