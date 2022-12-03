mod day1;
mod day10;
mod day11;
mod day12;
mod day14;
mod day15;
mod day16;
mod day18;
mod day19;
mod day2;
mod day22;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn day(n: i32) {
    match n {
        1 => day1::day(),
        2 => day2::day(),
        3 => day3::day(),
        4 => day4::day(),
        5 => day5::day(),
        6 => day6::day(),
        7 => day7::day(),
        8 => day8::day(),
        9 => day9::day(),
        10 => day10::day(),
        11 => day11::day(),
        12 => day12::day(),
        14 => day14::day(),
        15 => day15::day(),
        16 => day16::day(),
        18 => day18::day(),
        19 => day19::day(),
        22 => day22::day(),
        _ => panic!("day not implemented"),
    }
}
