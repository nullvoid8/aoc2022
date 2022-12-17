mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
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
        5 => run_on_input(day, day5::run, day5::parse),
        6 => run_on_input(day, day6::run, day6::parse),
        7 => run_on_input(day, day7::run, day7::parse),
        8 => run_on_input(day, day8::run, day8::parse),
        9 => run_on_input(day, day9::run, day9::parse),
        10 => run_on_input(day, day10::run, day10::parse),
        11 => run_on_input(day, day11::run, day11::parse),
        12 => run_on_input(day, day12::run, day12::parse),
        13 => run_on_input(day, day13::run, day13::parse),
        14 => run_on_input(day, day14::run, day14::parse),
        15 => run_on_input(day, day15::run, day15::parse),
        16 => run_on_input(day, day16::run, day16::parse),
        17 => run_on_input(day, day17::run, day17::parse),
        _ => Ok(()),
    }
}
