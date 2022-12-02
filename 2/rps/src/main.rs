use std::io::{self, BufRead};

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

#[derive(PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissor,
}

#[derive(PartialEq)]
enum Game {
    Lose,
    Draw,
    Win,
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
    return match (a, b) {
        (x, y) if x == y => Game::Draw,
        (RPS::Rock, RPS::Scissor) => Game::Win,
        (RPS::Scissor, RPS::Paper) => Game::Win,
        (RPS::Paper, RPS::Rock) => Game::Win,
        _ => Game::Lose,
    };
}

fn win_vs(a: RPS) -> RPS {
    return match a {
        RPS::Rock => RPS::Paper,
        RPS::Paper => RPS::Scissor,
        RPS::Scissor => RPS::Rock,
    };
}

fn lose_vs(a: RPS) -> RPS {
    return match a {
        RPS::Rock => RPS::Scissor,
        RPS::Paper => RPS::Rock,
        RPS::Scissor => RPS::Paper,
    };
}

fn draw_vs(a: RPS) -> RPS {
    return a;
}

fn score(s: &str) -> i32 {
    let score = match game(parse(s.chars().nth(2)), parse(s.chars().nth(0))) {
        Game::Win => 6,
        Game::Draw => 3,
        Game::Lose => 0,
    } + piece_score(parse(s.chars().nth(2)));
    return score;
}

fn piece_score(r: RPS) -> i32 {
    return match r {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissor => 3,
    };
}

fn score2(s: &str) -> i32 {
    let score = match parse2(s.chars().nth(2)) {
        Game::Win => 6 + piece_score(win_vs(parse(s.chars().nth(0)))),
        Game::Draw => 3 + piece_score(draw_vs(parse(s.chars().nth(0)))),
        Game::Lose => 0 + piece_score(lose_vs(parse(s.chars().nth(0)))),
    };
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
