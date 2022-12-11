use std::iter;

enum Command {
    Noop,
    Addx(i32),
}
use Command::*;

fn parse_cmd(s: &str) -> Command {
    match &s[0..4] {
        "noop" => Noop,
        "addx" => Addx(s[5..].parse::<i32>().unwrap()),
        _ => panic!(),
    }
}

fn eval(cs: &[Command]) -> impl Iterator<Item = (i32, i32)> + '_ {
    let rest = cs
        .iter()
        .scan((1, 1), |state, cmd| {
            let (cycle, x) = *state;
            match &cmd {
                Noop => {
                    *state = (cycle + 1, x);
                    Some(vec![*state].into_iter())
                }
                Addx(y) => {
                    *state = (cycle + 2, x + y);
                    Some(vec![(cycle + 1, x), *state].into_iter())
                }
            }
        })
        .flatten();
    iter::once((1, 1)).chain(rest)
}

fn render(input: &[Command]) {
    for (cycle, x) in eval(&input) {
        //        dbg!(cycle, x);
        //        let row = cycle / 40;
        let col = (cycle - 1) % 40;
        if col == 0 {
            println!();
        }
        if (col - x).abs() < 2 {
            print!("#");
        } else {
            print!(" ");
        }
    }
}

fn main() {
    let input: Vec<Command> = include_str!("../input.txt")
        .lines()
        .map(|line| parse_cmd(line))
        .collect();
    let mut signal_strength = 0;
    for (cycle, x) in eval(&input) {
        if cycle >= 20 && ((cycle - 20) % 40) == 0 {
            //            println!("{cycle:5} {x}");
            signal_strength += cycle * x;
        }
    }
    println!("{signal_strength}");
    render(&input);
}
