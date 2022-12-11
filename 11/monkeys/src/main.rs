#[derive(Clone, Eq, PartialEq, Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square(),
}
use Operation::*;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Test {
    Divisible { by: u64, tru: usize, fals: usize },
}
use Test::*;

impl Test {
    fn by(&self) -> u64 {
        match self {
            Divisible { by, .. } => *by,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    busy: u32,
}

fn last_number_of(s: &str) -> u64 {
    s.split(' ')
        .rev()
        .take(1)
        .next()
        .unwrap()
        .parse::<u64>()
        .unwrap()
}

impl Monkey {
    fn new(input: &str) -> Monkey {
        let mut input = input.lines();
        input.next(); // skip "Monkey 0:"
        let items = input
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect();
        let ops: Vec<&str> = input.next().unwrap().split(' ').rev().take(2).collect();
        let operation = if ops[0].trim() == "old" {
            Square()
        } else {
            match ops[1] {
                "+" => Add(ops[0].trim().parse::<u64>().unwrap()),
                "*" => Mul(ops[0].trim().parse::<u64>().unwrap()),
                _ => panic!(),
            }
        };
        let by = last_number_of(input.next().unwrap());
        let tru = last_number_of(input.next().unwrap());
        let fals = last_number_of(input.next().unwrap());
        Monkey {
            items: items,
            operation: operation,
            test: Divisible {
                by: by,
                tru: tru as usize,
                fals: fals as usize,
            },
            busy: 0,
        }
    }
    fn throw(&mut self, item: u64, part1: bool, product: u64) -> (usize, u64) {
        self.busy += 1;
        self.items = self.items[1..].to_vec();
        let item = match self.operation {
            Add(x) => item + x % product,
            Mul(x) => item * x % product,
            Square() => item * item % product,
        };
        let item = if part1 { item / 3 } else { item };
        match self.test {
            Divisible { by, tru, fals } => {
                if item % by == 0 {
                    (tru, item)
                } else {
                    (fals, item)
                }
            }
        }
    }
    fn receive(&mut self, item: u64) {
        self.items.push(item);
    }
}

fn run_game(monkeys: &mut [Monkey], part1: bool, rounds: usize) {
    let product = monkeys.iter().map(|m| m.test.by()).product();
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey_items = monkeys[i].items.clone();
            for item in monkey_items {
                let (dest, item) = monkeys[i].throw(item, part1, product);
                monkeys[dest].receive(item)
            }
        }
    }
    let mut busyness = vec![];
    for (_i, monkey) in monkeys.iter().enumerate() {
        busyness.push(monkey.busy);
    }
    busyness.sort();
    busyness.reverse();
    println!("{}", busyness[0] as u128 * busyness[1] as u128);
}

fn main() {
    let input: Vec<Monkey> = include_str!("../input")
        .split("\n\n")
        .map(|s| Monkey::new(s))
        .collect();
    let mut monkeys = input.clone();
    run_game(&mut monkeys, true, 20);
    let mut monkeys = input.clone();
    run_game(&mut monkeys, false, 10000);
}
