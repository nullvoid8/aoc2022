type Input = String;

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    Ok(input)
}

pub fn run(input: Input) -> () {
    println!("{}", process(&input, 4));
    println!("{}", process(&input, 14));
}

fn process(input: &str, n: usize) -> usize {
    input
        .as_bytes()
        .windows(n)
        .enumerate()
        .find_map(|(i, xs)| {
            let set = xs.iter().collect::<std::collections::HashSet<_>>();
            if set.len() != n {
                return None;
            }
            return Some(i + n);
        })
        .unwrap()
}
