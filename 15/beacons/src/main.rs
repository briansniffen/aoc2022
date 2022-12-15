use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    multi::many1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Zone {
    sensor: Point,
    beacon: Point,
}

impl Point {
    fn distance(&self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Zone {
    fn contains(&self, pt: Point) -> bool {
        self.sensor.distance(pt) <= self.sensor.distance(self.beacon)
    }
    fn min(&self) -> i32 {
        let dist = self.sensor.distance(self.beacon);
        self.sensor.x - dist
    }
    fn max(&self) -> i32 {
        let dist = self.sensor.distance(self.beacon);
        self.sensor.x + dist
    }
}

// Example:
// Sensor at x=2, y=18: closest beacon is at x=-2, y=15

fn parse_point(s: &str) -> IResult<&str, Point> {
    let (rest, (x, y)) = separated_pair(i32, tag(", y="), i32)(s)?;
    Ok((rest, Point { x, y }))
}

fn parse_sensor(s: &str) -> IResult<&str, Zone> {
    let (rest, (_, sensor, _, beacon, _)) = tuple((
        tag("Sensor at x="),
        parse_point,
        tag(": closest beacon is at x="),
        parse_point,
        newline,
    ))(s)?;
    Ok((rest, Zone { sensor, beacon }))
}

fn parse_sensors(s: &str) -> IResult<&str, Vec<Zone>> {
    many1(parse_sensor)(s)
}

fn part1(zones: &Vec<Zone>, y: i32) -> i32 {
    let x_min = zones.iter().map(|z| z.min()).min().unwrap();
    let x_max = zones.iter().map(|z| z.max()).max().unwrap();
    let mut count = 0;
    for x in x_min..=x_max {
        let pt = Point { x, y };
        if zones.iter().any(|z| z.sensor == pt || z.beacon == pt) {
            continue;
        };
        if zones.iter().any(|z| z.contains(pt)) {
            count += 1;
        }
    }
    count
}

fn part2(zones: &Vec<Zone>, max: i32) -> Point {
    for x in 0..=max {
        for y in 0..=max {
            let pt = Point { x, y };
            if zones.iter().any(|z| z.sensor == pt || z.beacon == pt) {
                continue;
            };
            if zones.iter().any(|z| z.contains(pt)) {
                continue;
            };
            return pt;
        }
    }
    return Point { x: 0, y: 0 };
}

fn main() {
    let input = include_str!("../input.txt");
    let (rest, sensors) = parse_sensors(input).unwrap();
    assert_eq!(rest, "");
    println!("{}", part1(&sensors, 2000000));
    println!("{:?}", part2(&sensors, 4000000));
}
