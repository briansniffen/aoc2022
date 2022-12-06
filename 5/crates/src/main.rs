use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, newline, u32};
use nom::combinator::{all_consuming, map, opt, value};
use nom::multi::{count, many0, many1, many1_count};
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

type Stack = Vec<char>;
type Stacks = Vec<Stack>;

struct Command {
    num: usize,
    from: usize,
    to: usize,
}

fn full(s: &str) -> IResult<&str, Option<char>> {
    map(
        terminated(delimited(char('['), anychar, char(']')), opt(char(' '))),
        Some,
    )(s)
}

fn empty(s: &str) -> IResult<&str, Option<char>> {
    value(None, alt((count(char(' '), 4), count(char(' '), 3))))(s)
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
    let (s, (_, num, _, from, _, to, _)) = tuple((
        tag("move "),
        u32,
        tag(" from "),
        u32,
        tag(" to "),
        u32,
        opt(newline),
    ))(s)?;
    Ok((
        s,
        Command {
            num: num as usize,
            from: from as usize,
            to: to as usize,
        },
    ))
}

fn input_file(s: &str) -> IResult<&str, (Stacks, Vec<Command>)> {
    let (s, (rows, num, _, cmds)) =
        all_consuming(tuple((many1(row), num_row, newline, many1(command))))(s)?;

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
    Ok((s, (stacks, cmds)))
}

fn main() {
    let input = include_str!("../input.txt");
    let (_input, (mut stacks, cmds)) = input_file(input).unwrap();
    for cmd in cmds {
        for _i in 0..cmd.num {
            let crane = stacks[cmd.from - 1].pop().unwrap();
            stacks[cmd.to - 1].push(crane);
        }
    }
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
    let (_input, (mut stacks, cmds)) = input_file(input).unwrap();
    for cmd in cmds {
        let index: usize = stacks[cmd.from - 1].len() - cmd.num;
        let mut crane = stacks[cmd.from - 1].split_off(index);
        assert_eq!(crane.len(), cmd.num);
        stacks[cmd.to - 1].append(&mut crane);
    }
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
