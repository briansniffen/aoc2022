use nom::character::complete::{char, u32};
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::IResult;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;

fn comma_sep(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    return separated_pair(range, char(','), range)(input);
}

fn range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    return map(separated_pair(u32, char('-'), u32), |(a, b)| (a..=b))(input);
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let count1 = lines
        .iter()
        .filter(|l| {
            let (_, (a, b)) = comma_sep(&l).unwrap();
            return (a.contains(&b.start()) && a.contains(&b.end()))
                || (b.contains(&a.start()) && b.contains(&a.end()));
        })
        .count();
    let count2 = lines
        .iter()
        .filter(|l| {
            let (_, (a, b)) = comma_sep(&l).unwrap();
            return (a.contains(&b.start()) || a.contains(&b.end()))
                || (b.contains(&a.start()) || b.contains(&a.end()));
        })
        .count();
    println!("{} {}", count1, count2);
}
