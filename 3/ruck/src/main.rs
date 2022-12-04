use std::collections::HashSet;
use std::io::{self, BufRead};

fn priority(item: u8) -> u8 {
    if (b'a'..(b'z'+1)).contains(&item) {
	return item - b'a' + 1;
    } else {
	// It's in A..Z by definition
	return item - b'A' + 27;
    }
}

fn ruck_set(v : Vec<u8>) -> HashSet<u8> {
    let mut h = HashSet::new();
    for b in v {
	h.insert(b);
    }
    return h;
}

fn main() {
    let stdin = io::stdin();
    let prio : u32 = stdin.lock().lines().map(|line| {
	let mut left: Vec<u8> = line.unwrap().bytes().collect();
	let right = left.split_off(left.len()/2);
	let left_set = ruck_set(left);
	let right_set = ruck_set(right);
	let common : Vec<&u8> = left_set.intersection(&right_set).collect();
	assert_eq!(common.len(), 1);
	return priority(**common.iter().next().unwrap()) as u32;
    }).sum();
    println!("{}", prio);
}
