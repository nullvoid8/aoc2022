mod day1;
mod day2;
mod day3;
mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;
// mod day10;
// mod day11;
// mod day12;
// mod day14;
// mod day15;
// mod day16;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
use get_inputs::run_on_input;

pub fn run_day(day: i32) -> Result<(), get_inputs::Error> {
    match day {
        1 => run_on_input(day, day1::run, day1::parse),
        2 => run_on_input(day, day2::run, day2::parse),
        3 => run_on_input(day, day3::run, day3::parse),
        4 => run_on_input(day, day4::run, day4::parse),
        _ => Ok(()),
    }
}
