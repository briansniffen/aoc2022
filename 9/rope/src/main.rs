use std::cmp::max;
use std::collections::HashSet;
use std::ops::{Add, Sub};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
    fn distance(&self, dest: Self) -> i32 {
        //self.abs_sub(&dest).magnitude()
        max((self.x - dest.x).abs(), (self.y - dest.y).abs())
    }
    fn move_towards(&mut self, dest: Self) {
        *self = *self + (dest - *self).signum();
    }
}

fn parse_dir(c: char) -> Point {
    match c {
        'U' => Point { x: 0, y: 1 },
        'D' => Point { x: 0, y: -1 },
        'L' => Point { x: -1, y: 0 },
        'R' => Point { x: 1, y: 0 },
        _ => panic!("bad direction"),
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut trail = HashSet::new();
    for line in input.lines() {
        let count = line[2..].parse::<i32>().expect("int");
        let dir = parse_dir(line.chars().nth(0).unwrap());
        for _i in 0..count {
            head = head + dir;
            if tail.distance(head) > 1 {
                tail.move_towards(head);
            }
            trail.insert(tail);
        }
    }
    println!("{}", trail.len());
    let mut knots = vec![Point { x: 0, y: 0 }; 10];
    trail.clear();
    for line in input.lines() {
        let count = line[2..].parse::<i32>().expect("int");
        let dir = parse_dir(line.chars().nth(0).unwrap());
        for _i in 0..count {
            knots[0] = knots[0] + dir;
            for i in 1..knots.len() {
                if knots[i].distance(knots[i - 1]) > 1 {
                    let dest = knots[i - 1];
                    knots[i].move_towards(dest);
                }
            }
            trail.insert(knots[9]);
        }
    }
    println!("{}", trail.len());
}
