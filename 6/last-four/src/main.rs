use std::collections::HashSet;
use std::str;

fn all_different(s: &[u8]) -> bool {
    let mut set = HashSet::new();
    for &c in s.iter() {
        set.insert(c);
    }
    set.len() == s.len()
}

fn find_first_difference(l: usize, input: &[u8]) {
    for i in l..input.len() {
        if all_different(&input[i - l..i]) {
            println!("{i} {}", str::from_utf8(&input[i - l..i]).unwrap());
            break;
        }
    }
}

fn main() {
    let input: Vec<u8> = include_str!("../input.txt").bytes().collect();
    find_first_difference(4, &input);
    find_first_difference(14, &input);
}
