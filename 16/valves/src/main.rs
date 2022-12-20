use petgraph::algo::dijkstra;
use petgraph::graphmap::GraphMap;
use petgraph::Directed;
use rayon::iter::ParallelBridge;
use rayon::prelude::*;
//use petgraph::dot::{Dot, Config};
use itertools::Itertools;
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
    path: &[&&'a str],
    valves: &HashMap<String, Valve>,
    paths: &HashMap<&'a str, HashMap<&'a str, u32>>,
    time: u32,
) -> u32 {
    let mut remaining = time;
    let mut score = 0;
    let mut last = "AA";
    for step in path {
        let cost = paths[last][*step];
        if cost > remaining {
            break;
        } else {
            remaining -= cost;
            score += valves[**step] * remaining;
            last = step;
        }
    }
    score
}

fn trunc<'a>(
    path: &[&&'a str],
    paths: &HashMap<&'a str, HashMap<&'a str, u32>>,
    time: u32,
) -> usize {
    let mut remaining = time;
    let mut last = "AA";
    for (i, step) in path.iter().enumerate() {
        let cost = paths[last][*step];
        if cost > remaining {
            return i;
        } else {
            remaining -= cost;
            last = step;
        }
    }
    path.len()
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
    let mut paths: HashMap<&str, HashMap<&str, u32>> = HashMap::new();
    for (label, valve) in &valves {
        if label != "AA" && *valve == 0 {
            continue;
        }
        let dijk = dijkstra(&graph, node_id(&label), None, |_| 1);
        let mut local_paths: HashMap<&str, u32> = HashMap::new();
        for (dest, cost) in &dijk {
            let dest = node_label(*dest, &valves);
            if dest == label || valves[dest] == 0 {
                continue;
            }
            local_paths.insert(dest, cost + 1); // we only go to a node to turn it on
        }
        paths.insert(label, local_paths);
    }
    let mut paths_sorted = paths
        .keys()
        .map(|&x| x)
        .filter(|&p| p != "AA")
        .collect::<Vec<_>>();
    paths_sorted.sort_by_key(|&x| valves[x]);

    // part 1
    let max_score = (0..8)
        .into_par_iter()
        .map(|i| {
            paths_sorted
                .iter()
                .rev()
                .permutations(i) // total hack, sorry
                .par_bridge()
                .map(|perm| score(&perm[0..trunc(&perm, &paths, 30)], &valves, &paths, 30))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("part 1: {}", max_score);

    // part 2
    let max_score = (0..12)
        .into_par_iter()
        .map(|i| {
            paths_sorted
                .iter()
                .rev()
                .permutations(i)
                .par_bridge()
                .map(|perm| {
                    (0..perm.len())
                        .into_iter()
                        .map(|chunk| {
                            score(&perm[0..chunk], &valves, &paths, 26)
                                + score(&perm[chunk..], &valves, &paths, 26)
                        })
                        .max()
                        .unwrap()
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("part 2: {}", max_score);
}
