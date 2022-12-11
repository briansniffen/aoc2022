use std::str::Lines;

enum Operation {
    Add(i32),
    Mul(i32),
}
use Operation::*;

enum Test {
    Divisible { by: i32, tru: usize, fals: usize },
}
use Test::*;

struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    test: Test,
    busy: u32,
}

fn parse_monkey(input: &str) -> Monkey {
    Monkey {
        items: vec![],
        operation: Add(0),
        test: Divisible {
            by: 3,
            tru: 0,
            fals: 0,
        },
        busy: 0,
    }
}

fn main() {
    let input = include_str!("../test")
        .split("\n\n")
        .map(|s| parse_monkey(s));
    println!("{}", input.count());
}
