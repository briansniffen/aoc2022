use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, newline, u32};
use nom::combinator::{map, value};
use nom::multi::{count, many0, many1, many1_count};
use nom::sequence::{delimited, terminated};
use nom::IResult;

type Stack = Vec<char>;
type Stacks = Vec<Stack>;

struct Command {
	num: u32,
	from: usize,
	to: usize,
}

fn full(s: &str) -> IResult<&str, Option<char>> {
	map(
		terminated(delimited(char('['), anychar, char(']')), char(' ')),
		Some,
	)(s)
}

fn empty(s: &str) -> IResult<&str, Option<char>> {
	value(None, count(char(' '), 4))(s)
}

fn row(s: &str) -> IResult<&str, Vec<Option<char>>> {
	terminated(many1(alt((full, empty))), newline)(s)
}

fn num_row(s: &str) -> IResult<&str, usize> {
	terminated(
		many1_count(delimited(many0(char(' ')), u32, many0(char(' ')))),
		newline,
	)(s)
}

fn command(s: &str) -> IResult<&str, Command> {
	let (s, _) = tag("move ")(s)?;
	let (s, num) = u32(s)?;
	let (s, _) = tag(" from ")(s)?;
	let (s, from) = u32(s)?;
	let (s, _) = tag(" to ")(s)?;
	let (s, to) = terminated(u32, newline)(s)?;
	Ok((
		s,
		Command {
			num: num,
			from: from as usize,
			to: to as usize,
		},
	))
}

fn input_file(s: &str) -> IResult<&str, (Stacks, Vec<Command>)> {
	let (s, rows) = many1(row)(s)?;
	let (s, num) = num_row(s)?;
	let (s, _) = newline(s)?;
	let (s, cmds) = many1(command)(s)?;

	// 'X' is a witness for char
	let mut stacks: Stacks = vec![vec![]; num];
	for row in rows.iter().rev() {
		for (i, x) in row.iter().enumerate() {
			match x {
				Some(x) => stacks[i].push(*x),
				None => (),
			}
		}
	}
	return Ok((s, (stacks, cmds)));
}

fn main() {
	let input = include_str!("../input.txt");
	let (_input, (mut stacks, cmds)) = input_file(input).unwrap();
	println!("{:?}", stacks);
	for cmd in cmds {
		for i in 0..cmd.num {
			let crane = stacks[cmd.from - 1].pop().unwrap();
			stacks[cmd.to - 1].push(crane);
		}
	}
	println!("{:?}", stacks);
	for stack in stacks {
		println!("{}", stack.last().unwrap());
	}
}
