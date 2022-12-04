type Input = Vec<(i32, i32)>;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

const LOSE: i32 = 1;
const DRAW: i32 = 2;
const WIN: i32 = 3;

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    Ok(input
        .lines()
        .map_while(|s| {
            let left = match s.chars().nth(0)? {
                'A' => ROCK,
                'B' => PAPER,
                'C' => SCISSORS,
                _ => return None,
            };

            let right = match s.chars().nth(2)? {
                'X' => ROCK,
                'Y' => PAPER,
                'Z' => SCISSORS,
                _ => return None,
            };
            Some((left, right))
        })
        .collect())
}

pub fn run(rounds: Input) {
    let score1 = rounds
        .iter()
        .map(|&(opponent, me)| match me - opponent {
            // Lose: R - P, P - S, S - R
            -1 | 2 => me,
            // Draw
            0 => 3 + me,
            // Win: R - S, P - R, S - P
            -2 | 1 => 6 + me,
            _ => 0,
        })
        .sum::<i32>();

    println!("{}", score1);

    let score2 = rounds
        .iter()
        .map(|&(opponent, goal)| match goal {
            LOSE => {
                0 + match opponent {
                    ROCK => SCISSORS,
                    PAPER => ROCK,
                    SCISSORS => PAPER,
                    _ => 0,
                }
            }
            DRAW => 3 + opponent,
            WIN => {
                6 + match opponent {
                    ROCK => PAPER,
                    PAPER => SCISSORS,
                    SCISSORS => ROCK,
                    _ => 0,
                }
            }
            _ => 0,
        })
        .sum::<i32>();

    println!("{}", score2);
}
