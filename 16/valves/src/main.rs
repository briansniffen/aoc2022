use petgraph::algo::floyd_warshall;
use petgraph::graphmap::GraphMap;
use petgraph::Directed;
use regex::Regex;
use std::collections::HashMap;

type Valve = u32;

fn node_id(s: &str) -> u32 {
    let mut s = s.bytes();
    s.next().unwrap() as u32 * 100 + s.next().unwrap() as u32
}

fn node_label<'a>(n: u32, valves: &'a HashMap<String, Valve>) -> &'a str {
    let first = (n / 100) as u8;
    let second = (n % 100) as u8;
    let label = std::str::from_utf8(&[first, second]).unwrap().to_owned();
    for key in valves.keys() {
        if key == &label {
            return key;
        };
    }
    panic!("label not found");
}

fn score<'a>(
    path: &[&'a str],
    valves: &HashMap<String, Valve>,
    paths: &HashMap<(&'a str, &'a str), u32>,
    time: u32,
) -> u32 {
    let mut remaining = time;
    let mut score = 0;
    let mut last = "AA";
    for step in path {
        if step == &last {
            continue;
        }
        let cost = paths[&(last, *step)];
        if cost > remaining {
            break;
        } else {
            remaining -= cost;
            score += valves[*step] * remaining;
            last = step;
        }
    }
    score
}

fn score2<'a>(
    path: &[&'a str],
    valves: &HashMap<String, Valve>,
    paths: &HashMap<(&'a str, &'a str), u32>,
    time: u32,
) -> (u32, u32) {
    let mut max = 0;
    let mut c = 0;
    for elephant in 1..(path.len() - 1) {
        let s = score(&path[..elephant], &valves, &paths, time)
            + score(&path[elephant..], &valves, &paths, time);
        if s == 2031 {
            dbg!(elephant);
        };
        if s > max {
            max = s;
            c = std::cmp::min(
                cost(&path[..elephant], &paths),
                cost(&path[elephant..], &paths),
            )
        };
    }
    (c, max)
}

fn cost<'a>(path: &[&'a str], paths: &HashMap<(&'a str, &'a str), u32>) -> u32 {
    let mut total = 0;
    let mut last = "AA";
    for step in path.iter() {
        if *step == last {
            continue;
        }
        total += paths[&(last, *step)];
        last = step;
    }
    total
}

fn explore<'a>(
    mut path: &mut [&'a str],
    sorted: &[&'a str],
    paths: &HashMap<(&'a str, &'a str), u32>,
    valves: &HashMap<String, Valve>,
    time: u32,
    index: usize,
) -> (u32, Vec<&'a str>) {
    let mut max = 0;
    let mut max_path = vec![];
    for node in sorted.iter().rev() {
        if path[0..index].contains(node) {
            continue;
        }
        path[index] = node;
        if index < path.len() - 1
            && index < sorted.len() - 1
            && cost(&path[0..=index], &paths) < time
        {
            let (score, new_max_path) =
                explore(&mut path, &sorted, &paths, &valves, time, index + 1);
            if score > max {
                max = score;
                max_path = new_max_path;
            }
        } else {
            let score = score(&path[0..=index], &valves, &paths, time);
            if score > max {
                max = score;
                max_path = path[0..=index].to_vec();
            }
        }
    }
    (max, max_path)
}

fn explore2<'a>(
    mut path: &mut [&'a str],
    sorted: &[&'a str],
    paths: &HashMap<(&'a str, &'a str), u32>,
    valves: &HashMap<String, Valve>,
    time: u32,
    index: usize,
) -> (u32, Vec<&'a str>) {
    let mut max = 0;
    let mut max_path = vec![];
    for node in sorted.iter().rev() {
        if path[0..index].contains(node) {
            continue;
        }
        path[index] = node;
        let (cost, score) = score2(&path[0..=index], &valves, &paths, time);
        if index < path.len() - 1 && index < sorted.len() - 1 && cost < time {
            let (score, new_max_path) =
                explore2(&mut path, &sorted, &paths, &valves, time, index + 1);
            if score > max {
                max = score;
                max_path = new_max_path;
            }
        } else {
            if score > max {
                max = score;
                max_path = path[0..=index].to_vec();
            }
        }
    }
    (max, max_path)
}

fn main() {
    let input = include_str!("../input.txt");
    let re = Regex::new(r"Valve (?P<label>..) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<exits>.*)").unwrap();
    let mut valves = HashMap::new();
    let mut graph: GraphMap<u32, (), Directed> = GraphMap::new();
    for cap in re.captures_iter(input) {
        let label = cap["label"].to_string();
        let valve = cap["rate"].parse::<u32>().unwrap();
        let exits = cap["exits"].split(", ").collect::<Vec<&str>>();
        graph.add_node(node_id(&label));
        for exit in &exits {
            graph.add_edge(node_id(&label), node_id(&exit), ());
        }
        valves.insert(cap["label"].to_string(), valve);
    }
    let mut paths = HashMap::new();
    for ((a, b), cost) in floyd_warshall(&graph, |_| 1).unwrap() {
        let a = node_label(a, &valves);
        let b = node_label(b, &valves);
        if a != "AA" && valves[a] == 0 {
            continue;
        }
        paths.insert((a, b), cost + 1); // turn on valves we visit
    }
    let mut paths_sorted = valves
        .keys()
        .map(|x| x.as_str())
        .filter(|&p| p != "AA" && valves[p] > 0)
        .collect::<Vec<&str>>();
    paths_sorted.sort_by_key(|x| valves[*x]);

    // part 1

    let mut path = vec!["AA"; 15];
    let max = explore(&mut path, &paths_sorted, &paths, &valves, 30, 0);
    println!("part 1: {:?}", max);

    // part 2

    let mut path = vec!["AA"; 15];
    let max = explore2(&mut path, &paths_sorted, &paths, &valves, 26, 0);
    // let mut path2 = vec!["AA"; 15];
    // let max = explore3(&mut path, &mut path2, &paths_sorted, &paths, &valves, 26, 0);
    println!("part 2: {:?}", max);
}
