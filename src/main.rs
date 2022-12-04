mod aoc_days;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    let day = args[1].parse::<i32>().unwrap();

    match aoc_days::run_day(day) {
        Err(err) => {
            println!("{}", err)
        }
        Ok(_) => {
            println!("{}", "done")
        }
    }
}
