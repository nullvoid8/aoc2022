use std::fs;

pub fn day() {
    let mut x = 0;
    let mut y = 0;
    let mut a = 0;

    for line in fs::read_to_string("inputs/day2")
        .expect("Couldn't find input")
        .lines()
    {
        let (cmd, dist) = line.split_once(' ').expect("invalid command");
        let dist = dist.parse::<i32>().expect("dist not a number");

        match cmd {
            "forward" => {
                x += dist;
                y += dist * a;
            }
            "down" => {
                a += dist;
            }
            "up" => {
                a -= dist;
            }
            _ => panic!("{}", "unknown command"),
        }
    }

    println!("{} {}", x * a, x * y);
}
