use std::{collections::HashMap, io::BufRead};

use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(anyhow::anyhow!("Unknown direction: {}", c)),
        }
    }
}

struct AdjacentNodes {
    left: String,
    right: String,
}
impl AdjacentNodes {
    fn next(&self, d: Direction) -> &str {
        match d {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

type Road = Vec<Direction>;
type Graph = HashMap<String, AdjacentNodes>;

lazy_static! {
    static ref EDGE_RE: regex::Regex = regex::Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
}
fn parse_graph(lines: impl Iterator<Item = String>) -> Graph {
    let mut graph = Graph::new();
    for line in lines {
        let captures = EDGE_RE.captures(&line).unwrap();
        let from = captures.get(1).unwrap().as_str().to_string();
        let to = AdjacentNodes {
            left: captures.get(2).unwrap().as_str().to_string(),
            right: captures.get(3).unwrap().as_str().to_string(),
        };

        graph.insert(from, to);
    }

    graph
}

pub fn eighth_task_1(f: impl BufRead) -> u64 {
    let mut lines = f.lines();
    let road = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(Direction::try_from)
        .collect::<Result<Road, _>>()
        .unwrap();
    //empty line
    lines.next();
    let graph = parse_graph(lines.map(|l| l.unwrap()));
    let mut steps = 0;
    let mut current = "AAA";
    loop {
        for d in &road {
            let next = graph.get(current).unwrap().next(*d);
            steps += 1;
            if next == "ZZZ" {
                return steps;
            }
            current = next;
        }
    }
}

fn gcd(x: u64, y: u64) -> u64 {
    let mut a;
    let mut b;
    if x >= y {
        a = x;
        b = y;
    } else {
        a = y;
        b = x;
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn gcm(nums: &[u64]) -> u64 {
    nums.into_iter()
        .copied()
        .reduce(|x, y| x * y / gcd(x, y))
        .unwrap()
}
pub fn eighth_task_2(f: impl BufRead) -> u64 {
    let mut lines = f.lines();
    let road = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(Direction::try_from)
        .collect::<Result<Road, _>>()
        .unwrap();
    //empty line
    lines.next();
    let graph = parse_graph(lines.map(|l| l.unwrap()));
    let starting_nodes: Vec<_> = graph
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|s| s.as_str())
        .collect();
    let mut loops = vec![];
    for node in &starting_nodes {
        let mut current = *node;

        let mut steps = 0;
        'outer: loop {
            for d in &road {
                let next = graph.get(current).unwrap().next(*d);
                steps += 1;
                if next.ends_with('Z') {
                    loops.push(steps);
                    break 'outer;
                }
                current = next;
            }
        }
    }
    println!("Loops {:?}", loops);
    gcm(&loops)
}
