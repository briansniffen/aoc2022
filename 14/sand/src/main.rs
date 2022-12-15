use nom::{
    bytes::complete::tag,
    character::complete::{char, newline, u32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::str;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

type Path = Vec<Point>;

struct Arena {
    origin: Point,
    max_row: usize,
    min_col: usize,
    max_col: usize,
    arena: Vec<Vec<u8>>,
}

fn parse_point(s: &str) -> IResult<&str, Point> {
    let (rest, (x, y)) = separated_pair(u32, char(','), u32)(s)?;
    Ok((
        rest,
        Point {
            x: x as usize,
            y: y as usize,
        },
    ))
}

fn parse_path(s: &str) -> IResult<&str, Path> {
    separated_list1(tag(" -> "), parse_point)(s)
}

fn parse_paths(s: &str) -> IResult<&str, Vec<Path>> {
    separated_list1(newline, parse_path)(s)
}

fn path_from(a: &Point, b: &Point) -> Vec<Point> {
    if a.x == b.x {
        let (a, b) = if a.y > b.y { (b, a) } else { (a, b) };
        (a.y..=b.y)
            .into_iter()
            .map(|y| Point { x: a.x, y })
            .collect()
    } else if a.y == b.y {
        let (a, b) = if a.x > b.x { (b, a) } else { (a, b) };
        (a.x..=b.x)
            .into_iter()
            .map(|x| Point { x: x, y: a.y })
            .collect()
    } else {
        panic!()
    }
}

impl Arena {
    fn new(max_row: usize, min_col: usize, max_col: usize) -> Self {
        let arena = vec![vec![b'.'; max_col - min_col + 5]; max_row + 3];
        Arena {
            origin: Point { x: 500, y: 0 }, // x is col and so goes *second*
            max_row,
            min_col: min_col - 2,
            max_col: max_col + 2,
            arena,
        }
    }
    fn pt(&self, x: usize, y: usize) -> u8 {
        let x = x - self.min_col;
        self.arena[y][x]
    }
    fn pt_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        let x = x - self.min_col;
        &mut self.arena[y][x]
    }
    fn petrify(&mut self, path: &Path) {
        for (a, b) in path.iter().zip(&mut path.iter().skip(1)) {
            for pt in path_from(&a, &b) {
                *self.pt_mut(pt.x, pt.y) = b'#';
            }
        }
    }
    fn petrify_floor(&mut self) {
        let y = self.max_row + 2;
        for x in self.min_col..=self.max_col {
            *self.pt_mut(x, y) = b'#';
        }
    }
    fn display(&self) {
        for row in &self.arena {
            println!("{}", str::from_utf8(&row).unwrap());
        }
        println!("");
    }
    fn simulate_sand(&mut self) -> usize {
        let mut count = 0;
        loop {
            count += 1;
            let mut grain = self.origin.clone();
            while self.pt(grain.x, grain.y) == b'.' {
                if grain.y == self.max_row+2 {
                    return count - 1; // part 1
                };
                if self.pt(grain.x, grain.y + 1) == b'.' {
                    grain = Point {
                        x: grain.x,
                        y: grain.y + 1,
                    };
                } else if self.pt(grain.x - 1, grain.y + 1) == b'.' {
                    grain = Point {
                        x: grain.x - 1,
                        y: grain.y + 1,
                    };
                } else if self.pt(grain.x + 1, grain.y + 1) == b'.' {
                    grain = Point {
                        x: grain.x + 1,
                        y: grain.y + 1,
                    };
                } else {
                    *self.pt_mut(grain.x, grain.y) = b'o';
                    if grain == self.origin {
                        return count; // part 2
                    };
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let (rest, paths) = parse_paths(input).unwrap();
    assert_eq!(rest, "\n");
    let min_col = paths.iter().flatten().map(|pt| pt.x).min().unwrap();
    let max_col = paths.iter().flatten().map(|pt| pt.x).max().unwrap();
    let max_row = paths.iter().flatten().map(|pt| pt.y).max().unwrap();
    let mut arena = Arena::new(max_row, min_col, max_col);
    for path in &paths {
        arena.petrify(path);
    }
    //arena.display();		
    let count = arena.simulate_sand();
    //arena.display();
    println!("{count}");

    let mut arena = Arena::new(max_row, 300, 700);
    for path in &paths {
        arena.petrify(path);
    }
    arena.petrify_floor();
    //arena.display();
    let count = arena.simulate_sand();
    //arena.display();
    println!("{count}");
}
