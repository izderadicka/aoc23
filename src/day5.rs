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

    pub fn map(&self, src: u64) -> u64 {
        let mut dst = src;
        for (range, offset) in &self.items {
            if range.contains(&src) {
                dst = (dst as i64 + offset) as u64;
            }
        }
        dst
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
    map
}

pub fn fifth_task_1(f: impl BufRead) -> u64 {
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

    find_min(seeds, maps)
}

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

    let mut min = u64::MAX;
    assert!(seeds.len() % 2 == 0);
    for i in (0..seeds.len()).step_by(2) {
        for seed in seeds[i]..seeds[i] + seeds[i + 1] {
            let mut val = seed;
            for map in &maps {
                let new_val = map.map(val);
                // println!("{} -> {}", val, new_val);
                val = new_val;
            }
            // println!("Mapping {} -> {}", seed, val);
            if val < min {
                min = val;
            }
        }
    }
    min
}

fn find_min(seeds: Vec<u64>, maps: Vec<Map>) -> u64 {
    let mut min = u64::MAX;
    for seed in seeds {
        let mut val = seed;
        for map in &maps {
            let new_val = map.map(val);
            println!("{} -> {}", val, new_val);
            val = new_val;
        }
        println!("Mapping {} -> {}", seed, val);
        if val < min {
            min = val;
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
        assert_eq!(map.map(98), 50);
        assert_eq!(map.map(99), 51);
        assert_eq!(map.map(50), 52);
        assert_eq!(map.map(97), 99);
        assert_eq!(map.map(49), 49);
        assert_eq!(map.map(100), 100);
        assert_eq!(map.map(10), 10);
    }
}
