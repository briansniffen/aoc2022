use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut elf = 0;
    let mut highest_elf = 0;
    let mut highest_total = 0;
    let mut running_total = 0;
    let mut totals = vec![];
    
    for line in stdin.lock().lines() {
	match line.expect("string").parse::<i32>() {
	    Ok(i) => {
		running_total += i;
	    },
	    Err(_s) => {
		elf += 1;
		totals.push(running_total);
		if running_total > highest_total {
		    highest_total = running_total;
		    highest_elf = elf;
		}
		running_total = 0;
	    },
	} 
    }
    println!("{} {}", highest_elf, highest_total);
    totals.sort();
    println!("{}", totals.iter().rev().take(3).sum::<i32>());
}
