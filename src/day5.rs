use std::{io::BufRead, ops::Range};

#[derive(Debug)]
struct Map {
    items: Vec<(Range<u64>, i64)>,
}

impl Map {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, dst_start: u64, src_start: u64, len: u64) {
        let offset: i64 = dst_start as i64 - src_start as i64;
        self.items.push((src_start..src_start + len, offset));
    }

    pub fn map(&self, src: &Range<u64>) -> Vec<Range<u64>> {
        let mut result = vec![];
        let mut elem = Some(src.clone());
        for (range, offset) in &self.items {
            if let Some(ref input) = elem {
                // end is before the range => map as identity as there is no mapping
                if input.end < range.start {
                    result.push(input.clone());
                    elem = None;
                    break;
                // start is after range => do nothing an continue
                } else if input.start >= range.end {
                    continue;
                } else {
                    let before = input.start..range.start;
                    if !before.is_empty() {
                        result.push(before);
                    }
                    let within = input.start..input.end.min(range.end);
                    if !within.is_empty() {
                        let mapped = (within.start as i64 + *offset) as u64
                            ..(within.end as i64 + *offset) as u64;
                        result.push(mapped);
                    }
                    let after = range.end..input.end;
                    if !after.is_empty() {
                        elem = Some(after);
                    } else {
                        elem = None;
                        break;
                    }
                }
            } else {
                break;
            }
        }

        if let Some(last) = elem {
            result.push(last);
        }

        result
    }

    pub fn sort(&mut self) {
        self.items.sort_by(|a, b| a.0.start.cmp(&b.0.start));
    }
}

fn collect_numbers(line: &str) -> Vec<u64> {
    line.split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_map(mut lines: impl Iterator<Item = String>, name: &str) -> Map {
    let header = lines.next().unwrap();
    let header = header.split(" ").next().unwrap();
    assert_eq!(header, name);
    let mut map = Map::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let numbers = collect_numbers(&line);

        map.add(numbers[0], numbers[1], numbers[2]);
    }
    map.sort();
    map
}

// pub fn fifth_task_1(f: impl BufRead) -> u64 {
//     let mut lines = f.lines().map(|l| l.unwrap());

//     let line = lines.next().unwrap();
//     let mut seeds_line = line.split(":");
//     assert_eq!(seeds_line.next().unwrap(), "seeds");
//     let seeds: Vec<u64> = seeds_line
//         .next()
//         .unwrap()
//         .trim()
//         .split(" ")
//         .map(|s| s.parse().unwrap())
//         .collect();

//     assert!(lines.next().unwrap().is_empty());
//     println!("Seeds: {:?}", seeds);
//     let maps = load_maps(lines);

//     find_min(seeds, maps)
// }

pub fn fifth_task_2(f: impl BufRead) -> u64 {
    let mut lines = f.lines().map(|l| l.unwrap());

    let line = lines.next().unwrap();
    let mut seeds_line = line.split(":");
    assert_eq!(seeds_line.next().unwrap(), "seeds");
    let seeds: Vec<u64> = seeds_line
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();

    assert!(lines.next().unwrap().is_empty());
    println!("Seeds: {:?}", seeds);
    let maps = load_maps(lines);
    assert!(seeds.len() % 2 == 0);
    let seeds = seeds.chunks(2).map(|c| c[0]..c[0] + c[1]).collect();
    let min = find_min(seeds, maps);
    min
}

fn find_min(mut seeds: Vec<Range<u64>>, maps: Vec<Map>) -> u64 {
    let mut min = u64::MAX;
    let mut results = vec![];
    for map in &maps {
        for seed in &seeds {
            let new_val = map.map(seed);
            results.extend(new_val);
        }
        println!("Mapping {:?} -> {:?}", seeds, results);
        seeds = results;
        results = vec![];
    }
    for seed in seeds {
        if seed.start < min {
            min = seed.start;
        }
    }
    min
}

fn load_maps(mut lines: impl Iterator<Item = String>) -> Vec<Map> {
    const MAP_NAMES: &[&str] = &[
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];
    let mut maps = vec![];
    for name in MAP_NAMES {
        let map = parse_map(&mut lines, name);
        println!("{}: {:?}", name, map);
        maps.push(map);
    }
    maps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let map_str = "seed-to-soil map:
50 98 2
52 50 48";
        let mut lines = map_str.lines().map(|l| l.to_string());
        let map = parse_map(&mut lines, "seed-to-soil");
        assert_eq!(map.map(&(94..100)), vec![96..100, 50..52]);
        assert_eq!(map.map(&(50..52)), vec![52..54]);
        assert_eq!(map.map(&(10..12)), vec![10..12]);
    }
}
