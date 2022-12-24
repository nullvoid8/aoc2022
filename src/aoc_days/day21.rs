use std::{
    collections::{BTreeMap, HashMap},
    ops::{Add, Div, Mul, Sub},
};

use inpt::{inpt, Inpt};

#[derive(Debug, Inpt, Clone, Copy)]
pub enum Op {
    #[inpt(regex = r"\+")]
    Add,
    #[inpt(regex = r"-")]
    Sub,
    #[inpt(regex = r"\*")]
    Mul,
    #[inpt(regex = r"/")]
    Div,
}

#[derive(Debug, Inpt, Clone)]
pub enum Job {
    #[inpt(regex = r"(\d+)")]
    Shout(i64),
    #[inpt(regex = r"(\pL{4}) (.) (\pL{4})")]
    Calc { left: String, op: Op, right: String },
}

#[derive(Debug, Inpt, Clone)]
#[inpt(regex = r"(\pL{4}): (.*)")]
pub struct Monkey {
    name: String,
    job: Job,
}

type Input = Vec<Monkey>;

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    Ok(inpt(&input).unwrap())
}

pub fn run(input: Input) -> () {
    let mut jobs: HashMap<String, Job> = HashMap::new();

    for Monkey { name, job } in input {
        jobs.insert(name, job);
    }

    let p1: i64 = calc_monkey(&jobs, &mut HashMap::new(), "root");
    println!("{p1}");

    match jobs.get_mut("root").unwrap() {
        Job::Shout(_) => panic!(),
        Job::Calc { op, .. } => *op = Op::Sub,
    }

    let mut memo: HashMap<String, FloatingMonkey> =
        [("humn".to_owned(), FloatingMonkey([(1, 1 as f64)].into()))].into();
    let (left, right) = match jobs.get("root").unwrap() {
        Job::Shout(_) => panic!(),
        Job::Calc { left, right, .. } => (left, right),
    };
    let left = calc_monkey(&jobs, &mut memo, &left);
    let right = calc_monkey(&jobs, &mut memo, &right);

    let root = left - right;
    // k * x + c = 0 => x = -c/k
    let p2 = -root.0[&0] / root.0[&1];
    println!("{p2}");
}

fn calc_monkey<
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<i64> + Clone,
>(
    jobs: &HashMap<String, Job>,
    memo: &mut HashMap<String, T>,
    key: &str,
) -> T {
    if let Some(x) = memo.get(key) {
        return x.clone();
    }

    let job = jobs.get(key).unwrap().clone();
    match job {
        Job::Shout(x) => x.into(),
        Job::Calc { left, op, right } => {
            let left = calc_monkey(jobs, memo, &left);
            let right = calc_monkey(jobs, memo, &right);
            let val = match op {
                Op::Add => left + right,
                Op::Sub => left - right,
                Op::Mul => left * right,
                Op::Div => left / right,
            };
            memo.insert(key.to_owned(), val.clone());
            val
        }
    }
}

// {pow: k} => sum(k * humn^pow)
#[derive(Debug, Default, Clone)]
struct FloatingMonkey(BTreeMap<u32, f64>);

impl From<i64> for FloatingMonkey {
    fn from(x: i64) -> Self {
        FloatingMonkey([(0, x as f64)].into())
    }
}

impl std::ops::Add for FloatingMonkey {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = self.clone();
        for (pow, k) in rhs.0 {
            *out.0.entry(pow).or_default() += k;
        }
        out
    }
}

impl std::ops::Sub for FloatingMonkey {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = self.clone();
        for (pow, k) in rhs.0 {
            *out.0.entry(pow).or_default() -= k;
        }
        out
    }
}

impl std::ops::Mul for FloatingMonkey {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Self::default();
        for (kl, vl) in self.0 {
            for (kr, vr) in &rhs.0 {
                *out.0.entry(kl + kr).or_default() += vl * vr;
            }
        }
        out
    }
}

impl std::ops::Div for FloatingMonkey {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut out = Self::default();
        for (kl, vl) in self.0 {
            for (kr, vr) in &rhs.0 {
                *out.0.entry(kl - kr).or_default() += vl / vr;
            }
        }
        out
    }
}
