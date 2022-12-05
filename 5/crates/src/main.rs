use nom::character::complete::{char, anychar, u32,newline,multispace0};
use nom::bytes::complete::tag;
use nom::sequence::{delimited,terminated};
use nom::combinator::{map,value};
use nom::multi::{count,many1,many1_count};
use nom::branch::alt;
use nom::{IResult,error::ParseError};

type Stack = Vec<char>;
type Stacks = Vec<Stack>;

struct Command{num: u32, from: u32, to: u32}


fn full(s: &str) -> IResult<&str, Option<char>> {
    map(delimited(char('['),
		  anychar,
		  char(']')),

	Some)(s)
}

fn empty(s: &str) -> IResult<&str,Option<char>>{
    value(None,count(char(' '), 3))(s)
}

fn row(s: &str) -> IResult<&str,Vec<Option<char>>>{
    terminated(many1(alt((ws(full),ws(empty)))),
	       newline)(s)
}

fn num_row(s: &str) -> IResult<&str,usize> {
    println!("parsing num_row");
    terminated(many1_count(ws(u32)),
	       newline)(s)
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    multispace0,
    inner,
    multispace0
  )
}

fn command(s: &str) -> IResult<&str,Command> {
    let (s,_) = tag("move")(s)?;
    let (s,num) = ws(u32)(s)?;
    let (s,_) = tag("from")(s)?;
    let (s,from) = ws(u32)(s)?;
    let (s,_) = tag("to")(s)?;
    let (s,to) = terminated(ws(u32),
			    newline)(s)?;
    Ok((s,Command{num: num, from: from, to: to}))
}

fn input_file(s: &str) -> IResult<&str,(Stacks,Vec<Command>)> {
    let (s,rows) = many1(row)(s)?;
    println!("done with picture");
    let (s,num) = num_row(s)?;
    println!("done with numrow");
    let (s,cmds) = many1(command)(s)?;

    // 'X' is a witness for char
    let mut stacks : Stacks = vec![vec![];num];
    for row in rows.iter().rev() {
	for (i,x) in row.iter().enumerate() {
	    match x {
		Some(x) => stacks[i].push(*x),
		None => ()
	    }
	}
    }
    return Ok((s,(stacks,cmds)))
}

fn main() {
    let input = include_str!("../test");
    let (_input,(stacks,cmds)) = input_file(input).unwrap();
    for stack in stacks {
	println!("{}", stack.last().unwrap());
    }
    for cmd in cmds {
	println!("{} {} {}", cmd.num, cmd.from, cmd.to);
    }
}
