type Input = Vec<Vec<i32>>;

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    Ok(input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|food| food.parse::<i32>().unwrap())
                .collect()
        })
        .collect())
}

pub fn run(input: Input) -> () {
    let mut nums: Vec<i32> = input.iter().map(|elf| elf.iter().sum()).collect();

    nums.sort();
    nums.reverse();
    println!("{}", nums[0]);
    println!("{}", nums[0..3].iter().sum::<i32>())
}
