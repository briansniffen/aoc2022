use std::cmp::max;
use std::collections::HashSet;
use std::ops::{Add, Sub};
use std::{thread, time};
use termion::{clear, cursor};

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

fn walk_rope(input: &str, length: usize) -> Vec<Vec<Point>> {
    let mut trail = HashSet::new();
    let mut knots = vec![Point { x: 0, y: 0 }; length + 1];
    let mut log = vec![];
    for line in input.lines() {
        let count = line[2..].parse::<i32>().expect("int");
        let dir = parse_dir(line.chars().next().unwrap());
        for _i in 0..count {
            knots[0] = knots[0] + dir;
            for i in 1..knots.len() {
                if knots[i].distance(knots[i - 1]) > 1 {
                    let dest = knots[i - 1];
                    knots[i].move_towards(dest);
                }
            }
            trail.insert(knots[length]);
            log.push(knots.clone());
        }
    }
    println!("{}", trail.len());
    log
}

fn display_frame(frame: &[Point], sw_point: Point, ne_point: Point) {
    for row in sw_point.y..ne_point.y {
        for col in sw_point.x..ne_point.x {
            match frame.iter().position(|pt| *pt == Point { x: col, y: row }) {
                Some(i) => print!("{i}"),
                None => {
                    if col == 0 && row == 0 {
                        print!("s")
                    } else {
                        print!(".")
                    }
                }
            }
        }
        println!();
    }
}

fn debug_frame(frame: &[Point], sw_point: Point, ne_point: Point) {
    for pt in frame {
        print!("({},{}) ", pt.x, pt.y);
    }
    println!("");
}

fn animate_log(log: Vec<Vec<Point>>) {
    let min_x = log.iter().flatten().min_by_key(|pt| pt.x).unwrap().x;
    let min_y = log.iter().flatten().min_by_key(|pt| pt.y).unwrap().y;
    let max_x = log.iter().flatten().max_by_key(|pt| pt.x).unwrap().x;
    let max_y = log.iter().flatten().max_by_key(|pt| pt.y).unwrap().y;
    let sw_point = Point { x: min_x, y: min_y };
    let ne_point = Point { x: max_x, y: max_y };
    println!("{}", clear::All);
    for frame in log {
        println!("{} {}", cursor::Hide, cursor::Goto(1, 1),);
        display_frame(&frame, sw_point, ne_point);
        //       thread::sleep(time::Duration::from_millis(10));
    }
    println!("{}", cursor::Show);
}

fn main() {
    let input = include_str!("../input.txt");
    //animate_log(walk_rope(input, 1));
    animate_log(walk_rope(input, 9));
}
