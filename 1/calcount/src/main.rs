use itertools::Itertools;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let elfs = stdin
        .lock()
        .lines()
        .group_by(|line| line.as_ref().expect("string").len() == 0);
    let mut totals = elfs
        .into_iter()
        .filter(|(k, _)| !k)
        .map(|(_, elf)| {
            elf.map(|line| line.expect("string").parse::<i32>().expect("int"))
                .sum::<i32>()
        })
        .collect_vec();
    totals.sort();
    println!("{}", totals.iter().rev().take(1).sum::<i32>());
    println!("{}", totals.iter().rev().take(3).sum::<i32>());
}
