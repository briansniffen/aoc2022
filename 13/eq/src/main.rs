use nom::{
    branch::alt,
    character::complete::{char, newline, u32},
    combinator::opt,
    multi::separated_list0,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Clone, Debug)]
enum AOC {
    One(u32),
    Many(Vec<AOC>),
}
use AOC::*;

impl PartialOrd for AOC {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (One(a), One(b)) => a.partial_cmp(b),
            (Many(a), Many(b)) => Some(compare_aoc_lists(&a, &b)),
            (One(a), b) => Many(vec![One(*a)]).partial_cmp(b),
            (a, One(b)) => a.partial_cmp(&Many(vec![One(*b)])),
        }
    }
}

fn compare_aoc_lists(a: &[AOC], b: &[AOC]) -> Ordering {
    match (a, b) {
        ([], []) => Ordering::Equal,
        (_a, []) => Ordering::Greater,
        ([], _b) => Ordering::Less,
        (a, b) => {
            if a[0] == b[0] {
                compare_aoc_lists(&a[1..], &b[1..])
            } else {
                a.cmp(b)
            }
        }
    }
}

impl Ord for AOC {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (One(a), One(b)) => a.cmp(b),
            (Many(a), Many(b)) => compare_aoc_lists(&a, &b),
            (One(a), b) => Many(vec![One(*a)]).cmp(b),
            (a, One(b)) => a.cmp(&Many(vec![One(*b)])),
        }
    }
}

fn parse_datum(s: &str) -> IResult<&str, AOC> {
    match u32(s) {
        Ok((rest, s)) => Ok((rest, One(s))),
        Err(e) => Err(e),
    }
}

fn parse_list(s: &str) -> IResult<&str, AOC> {
    match delimited(char('['), separated_list0(char(','), parse_data), char(']'))(s) {
        Ok((rest, s)) => Ok((rest, Many(s))),
        Err(e) => Err(e),
    }
}

fn parse_data(s: &str) -> IResult<&str, AOC> {
    alt((parse_datum, parse_list))(s)
}

fn parse_file1(s: &str) -> IResult<&str, Vec<(AOC, AOC)>> {
    separated_list0(
        tuple((newline, newline)),
        separated_pair(parse_data, newline, parse_data),
    )(s)
}

fn parse_file2(s: &str) -> IResult<&str, Vec<AOC>> {
    separated_list0(tuple((newline, opt(newline))), parse_data)(s)
}

fn main() {
    let input = include_str!("../input");
    match parse_file1(input) {
        Ok(("\n", pairs)) => println!(
            "{}",
            pairs
                .iter()
                .enumerate()
                .map(|(i, (a, b))| if a < b { i + 1 } else { 0 })
                .sum::<usize>()
        ),
        Ok((rest, _pairs)) => println!("leftover {}", rest),
        Err(e) => panic!("{}", e),
    };
    let mut prefix = "[[2]]\n[[6]]\n".to_owned();
    prefix.push_str(input);
    match parse_file2(&prefix) {
        Ok(("\n", data)) => {
            let mut data = data.clone();
            data.sort();
            println!(
                "{}",
                data.iter()
                    .enumerate()
                    .map(|(i, x)| {
                        if Many(vec![Many(vec![One(2)])]) == *x
                            || Many(vec![Many(vec![One(6)])]) == *x
                        {
                            i + 1
                        } else {
                            1
                        }
                    })
                    .product::<usize>(),
            );
        }
        Ok((rest, _data)) => println!("leftover {} ", rest),
        Err(e) => panic!("{}", e),
    }
}
