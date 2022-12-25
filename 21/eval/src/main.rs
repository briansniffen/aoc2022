use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{char, i64, newline};
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}
use Op::*;

impl Op {
    fn apply(&self, a: i64, b: i64) -> i64 {
        let v = match self {
            Add => a + b,
            Sub => a - b,
            Mul => a * b,
            Div => a / b,
            Eq => panic!(),
        };
        v
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Monkey {
    Val { v: i64 },
    Expr { a: String, op: Op, b: String },
    Unk,
}
use Monkey::*;

fn label(s: &str) -> IResult<&str, &str> {
    take(4usize)(s)
}

fn add(s: &str) -> IResult<&str, Op> {
    match char('+')(s) {
        Ok((rest, _res)) => Ok((rest, Add)),
        Err(e) => Err(e),
    }
}
fn sub(s: &str) -> IResult<&str, Op> {
    match char('-')(s) {
        Ok((rest, _res)) => Ok((rest, Sub)),
        Err(e) => Err(e),
    }
}
fn mul(s: &str) -> IResult<&str, Op> {
    match char('*')(s) {
        Ok((rest, _res)) => Ok((rest, Mul)),
        Err(e) => Err(e),
    }
}
fn div(s: &str) -> IResult<&str, Op> {
    match char('/')(s) {
        Ok((rest, _res)) => Ok((rest, Div)),
        Err(e) => Err(e),
    }
}

fn op(s: &str) -> IResult<&str, Op> {
    alt((add, sub, mul, div))(s)
}

fn monkey_expr(s: &str) -> IResult<&str, Monkey> {
    let (rest, (a, _, op, _, b, _)) =
        tuple((label, char(' '), op, char(' '), label, opt(newline)))(s)?;
    Ok((
        rest,
        Expr {
            a: a.to_string(),
            op,
            b: b.to_string(),
        },
    ))
}

fn monkey_val(s: &str) -> IResult<&str, Monkey> {
    let (rest, (v, _)) = tuple((i64, opt(newline)))(s)?;
    Ok((rest, Val { v }))
}

fn monkey(s: &str) -> IResult<&str, (&str, Monkey)> {
    let (rest, (name, _, monkey)) = tuple((label, tag(": "), alt((monkey_val, monkey_expr))))(s)?;
    Ok((rest, (name, monkey)))
}

fn monkeys(s: &str) -> IResult<&str, HashMap<String, Monkey>> {
    let mut env = HashMap::new();
    let (rest, monkeys) = many1(monkey)(s)?;
    for (name, monkey) in monkeys {
        env.insert(name.to_owned(), monkey);
    }
    Ok((rest, env))
}

fn simplify(env: &mut HashMap<String, Monkey>) {
    let mut progress = false;
    loop {
        let labels = env.keys().cloned().collect::<Vec<_>>();
        for label in labels.into_iter() {
            let mut cell = None;
            match &env[&label] {
                Expr { a, op, b } => {
                    cell = match (&env[a], &env[b]) {
                        (Val { v: a }, Val { v: b }) => Some(op.apply(*a, *b)),
                        _ => None,
                    }
                }
                _ => (),
            };
            match cell {
                None => (),
                Some(v) => {
                    progress = true;
                    *env.get_mut(&label).unwrap() = Val { v }
                }
            }
        }
        if !progress {
            break;
        } else {
            progress = false;
        };
    }
}

//

fn invert(op: &Op, known: &i64, val: &i64, flip: bool) -> i64 {
    dbg!(op, known, val, flip);
    match (op, flip) {
        (Add, false) => val - known,
        (Add, true) => val - known,
        (Sub, false) => known - val,
        (Sub, true) => val + known,
        (Mul, false) => val / known,
        (Mul, true) => val / known,
        (Div, false) => known / val,
        (Div, true) => known * val,
        (Eq, _) => *known,
    }
}

fn main() {
    let input = include_str!("../input.txt");

    //part 1
    let (rest, mut env) = monkeys(input).unwrap();
    assert_eq!(rest, "");
    simplify(&mut env);
    match env["root"] {
        Val { v } => println!("part1: {}", v),
        _ => panic!("error in part 1"),
    }

    //part 2
    let (rest, mut env) = monkeys(input).unwrap();
    assert_eq!(rest, "");
    *env.get_mut("humn").unwrap() = Unk;
    match env.get_mut("root").unwrap() {
        Expr { ref mut op, .. } => *op = Eq,
        _ => panic!(),
    }
    simplify(&mut env);
    let mut point = "root";
    let mut val = 0;
    loop {
        match &env[point] {
            Unk => break,
            Val { .. } => panic!(),
            Expr { op, a, b } => {
                match &env[a] {
                    Val { v } =>
                    // v is known, b is unknown
                    {
                        dbg!(a);
                        val = invert(op, v, &val, false);
                        point = b;
                    }
                    _ => (),
                };
                match &env[b] {
                    Val { v } =>
                    // v is known, a is unknown
                    {
                        dbg!(b);
                        val = invert(op, v, &val, true);
                        point = a;
                    }
                    _ => (),
                }
            }
        }
    }
    println!("part 2: {val}");
}
