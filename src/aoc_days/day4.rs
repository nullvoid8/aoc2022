use std::fs;

use itertools::Itertools;
use nom::character::complete::{char, i32, line_ending, space0};

use nom::combinator::{eof, map};
use nom::multi::{count, many1, separated_list1};
use nom::sequence::{pair, preceded, terminated};
use nom::IResult;

type Board = [[Option<i32>; 5]; 5];

// int {',' int}
fn parse_list(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(char(','), i32)(input)
}

fn wrapped_i32(i: &str) -> IResult<&str, Option<i32>> {
    map(i32, Some)(i)
}

// newline int {{' '} int}
fn parse_row(input: &str) -> IResult<&str, [Option<i32>; 5]> {
    map(
        preceded(line_ending, count(preceded(space0, wrapped_i32), 5)),
        |x| x.try_into().unwrap(),
    )(input)
}

// newline {row}
fn parse_board(input: &str) -> IResult<&str, Board> {
    map(preceded(line_ending, count(parse_row, 5)), |x| {
        x.try_into().unwrap()
    })(input)
}

// list board
fn parse_input(input: &str) -> IResult<&str, (Vec<i32>, Vec<Board>)> {
    terminated(pair(parse_list, many1(parse_board)), eof)(input)
}

fn mark_number(pull: i32, board: Board) -> Board {
    let mut next = [[None; 5]; 5];
    for x in 0..5 {
        for y in 0..5 {
            if board[x][y] != Some(pull) {
                next[x][y] = board[x][y]
            }
        }
    }
    next
}

fn is_winner(board: &Board) -> bool {
    for i in 0..5 {
        if (board[i][0].is_none()
            && board[i][1].is_none()
            && board[i][2].is_none()
            && board[i][3].is_none()
            && board[i][4].is_none())
            || (board[0][i].is_none()
                && board[1][i].is_none()
                && board[2][i].is_none()
                && board[3][i].is_none()
                && board[4][i].is_none())
        {
            return true;
        }
    }

    false
}

fn calc_score(pull: i32, board: Board) -> i32 {
    pull * board
        .into_iter()
        .map(|r| r.into_iter().map(|x| x.unwrap_or(0)).sum::<i32>())
        .sum::<i32>()
}

pub fn day() {
    let file = fs::read_to_string("inputs/day4").expect("Couldn't find input");
    let (_, (pulls, boards)) = parse_input(&file).unwrap();

    let mut winners: Vec<i32> = Vec::new();
    let mut boards = boards;

    for pull in pulls {
        if boards.is_empty() {
            break;
        }
        boards = boards
            .into_iter()
            .map(|b| mark_number(pull, b))
            .collect_vec();

        let (new_winners, losers): (Vec<Board>, Vec<Board>) =
            boards.iter().partition(|b| is_winner(&b));
        winners.extend(new_winners.into_iter().map(|b| calc_score(pull, b)));
        boards = losers;
    }

    println!("{} {}", winners.first().unwrap(), winners.last().unwrap())
}
