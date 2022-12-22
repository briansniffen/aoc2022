use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{char, i64, newline};
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

use std::collections::HashMap;

#[derive(PartialEq,Eq,Debug,Copy,Clone)]
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

#[derive(PartialEq,Eq,Debug)]
enum Monkey {
    Val { v: i64 },
    Expr { a: String, op: Op, b: String },
    Unk,
}
use Monkey::*;

impl Monkey {
    fn val(&self) -> Option<i64> {
	match self {
	    Val{v} => Some(*v),
	    _ => None
	}
    }
    fn op(&self) -> Option<Op> {
	match self {
	    Expr{op,..} => Some(*op),
	    _ => None
	}
    }
}

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
    let (rest, (v,_)) = tuple((i64,opt(newline)))(s)?;
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

// n apply3(mut env: &HashMap<String,Monkey>, name: &str, left: &str, op: Op, right: &str) {
//     if name=="humn" {return;};
//     let mut change=None;
//     let left = &env[left];
//     let right = &env[right];
//     if op==Eq {
//     }
//     match (left,right) {
// 	(Val { v: a }, Val { v: b }) => {
// 	    change=Some(name, op.apply(*a, *b));
// 	},
// 	(Expr {..}, Expr {..}) => change=None,
// 	(Val {v: left}, Expr( a: right_a, op: right_op, b: right_b)) => {
	    
// 	}
    
    
// }

fn main() {
    let input = include_str!("../input.txt");
    let (rest, mut env) = monkeys(input).unwrap();
    assert_eq!(rest, "");

    // part 1
    'part1: loop {
	let labels = env.keys().cloned().collect::<Vec<_>>();
        for label in labels.into_iter() {
	    let mut cell=None;
            match &env[&label]{
                Val { .. } => {
                    if label == "root" {
                        break 'part1;
                    }
                },
                Expr { a, op, b } => {
                    cell = match (&env[a], &env[b]) {
                        (Val { v: a }, Val { v: b }) => Some(op.apply(*a, *b)),
                        _ => None,
                    }
                },
		Unk => panic!(),
            };
	    match cell {
		None => (),
		Some(v) => *env.get_mut(&label).unwrap() = Val{v},
	    }
        }
    }
    match env["root"] {
	Val{v} =>
	    println!("part1: {}", v),
	_ => panic!("error in part 1"),
    }



    // modify humn to Unk
    // modify root to Eq
    // 'part2: loop {
    // 	let labels = env.keys().cloned().collect::<Vec<_>>();
    //     for label in labels.into_iter() {
	    


    //     }
    // }
    // match env["humn"] {
    // 	Val{v} =>
    // 	    println!("part2: {}", v),
    // 	_ => panic!("error in part 2"),
    // }
    
}
