use itertools::Itertools;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn priority(item: u8) -> u8 {
    if (b'a'..(b'z' + 1)).contains(&item) {
        return item - b'a' + 1;
    } else {
        // It's in A..Z by definition
        return item - b'A' + 27;
    }
}

fn ruck_set(v: &[u8]) -> HashSet<u8> {
    let mut h = HashSet::new();
    for b in v {
        h.insert(*b);
    }
    return h;
}

fn main() {
    let stdin = io::stdin();
    let buffer = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    let squads = buffer.iter().chunks(3);
    let prio: u32 = squads
        .into_iter()
        .map(|squad_iter| {
            let sets: Vec<HashSet<u8>> = squad_iter
                .map(|elf| ruck_set(&elf.bytes().collect::<Vec<u8>>()))
                .collect();
            let first_two: Vec<u8> = sets[0].intersection(&sets[1]).copied().collect();
            let badge: Vec<u8> = ruck_set(&first_two)
                .intersection(&sets[2])
                .copied()
                .collect();
            return priority(*badge.iter().next().unwrap()) as u32;
        })
        .sum();
    println!("{}", prio);
}
