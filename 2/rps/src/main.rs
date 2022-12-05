use std::io::{self, BufRead};
extern crate num;
#[macro_use]
extern crate num_derive;

// The winner of the whole tournament is the player with the highest
// score. Your total score is the sum of your scores for each
// round. The score for a single round is the score for the shape you
// selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the
// score for the outcome of the round (0 if you lost, 3 if the round
// was a draw, and 6 if you won).

// X 1
// Y 2
// Z 3
// lose 0
// draw 3
// win  6

#[derive(PartialEq, FromPrimitive)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[derive(PartialEq, FromPrimitive)]
enum Game {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn parse(c: Option<char>) -> RPS {
    return match c.unwrap() {
        'A' | 'X' => RPS::Rock,
        'B' | 'Y' => RPS::Paper,
        'C' | 'Z' => RPS::Scissor,
        _ => panic!("whoops {}", c.unwrap()),
    };
}

fn parse2(c: Option<char>) -> Game {
    return match c.unwrap() {
        'X' => Game::Lose,
        'Y' => Game::Draw,
        'Z' => Game::Win,
        _ => panic!("whoops {}", c.unwrap()),
    };
}

// game state for player a
fn game(a: RPS, b: RPS) -> Game {
    let win = num::FromPrimitive::from_i32(((a as i32 - b as i32 + 4) % 3) * 3).unwrap();
    return win;
}

fn play_vs(need: Game, them: RPS) -> RPS {
    return num::FromPrimitive::from_i32(
        ((((them as i32 + ((need as i32 - 3) / 3) - 1) % 3) + 3) % 3) + 1,
    )
    .unwrap();
}

fn score(s: &str) -> i32 {
    return game(parse(s.chars().nth(2)), parse(s.chars().nth(0))) as i32
        + parse(s.chars().nth(2)) as i32;
}

fn score2(s: &str) -> i32 {
    let score = (parse2(s.chars().nth(2)) as i32)
        + play_vs(parse2(s.chars().nth(2)), parse(s.chars().nth(0))) as i32;
    return score;
}

fn main() {
    let stdin = io::stdin();
    let (puzzle1, puzzle2): (Vec<i32>, Vec<i32>) = stdin
        .lock()
        .lines()
        .map(|line| {
            let l = &line.expect("line");
            (score(l), score2(l))
        })
        .unzip();
    println!(
        "{} {}",
        puzzle1.iter().sum::<i32>(),
        puzzle2.iter().sum::<i32>()
    );
}
