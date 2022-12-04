pub struct Range {
    lower: i32,
    upper: i32,
}

impl Range {
    fn new_incl(lower: i32, upper: i32) -> Range {
        Range {
            lower,
            upper: upper + 1,
        }
    }

    fn includes(&self, other: &Self) -> bool {
        // a-x-y-b
        self.lower <= other.lower && other.upper <= self.upper
    }

    fn overlaps(&self, other: &Self) -> bool {
        // ! a--b x--y
        // ! x--y a--b
        !(other.upper <= self.lower) && !(self.upper <= other.lower)
    }
}

type Input = Vec<(Range, Range)>;

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    let mut rangesets: Input = Vec::new();

    for line in input.lines() {
        let (a, line) = line.split_once("-").unwrap();
        let (b, line) = line.split_once(",").unwrap();
        let (c, d) = line.split_once("-").unwrap();

        rangesets.push((
            Range::new_incl(a.parse().unwrap(), b.parse().unwrap()),
            Range::new_incl(c.parse().unwrap(), d.parse().unwrap()),
        ))
    }

    Ok(rangesets)
}

pub fn run(input: Input) -> () {
    let enclosed = input
        .iter()
        .filter(|(a, b)| a.includes(b) || b.includes(a))
        .count();
    println!("{}", enclosed);

    let overlaps = input.iter().filter(|(a, b)| a.overlaps(b)).count();

    println!("{}", overlaps);
}
