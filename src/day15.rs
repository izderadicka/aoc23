use std::{collections::HashMap, io::BufRead, str::FromStr};

use anyhow::Context;

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

#[derive(Debug)]
struct Carton {
    lens: Vec<String>,
    lens_values: HashMap<String, u8>,
}

impl Carton {
    fn new() -> Self {
        Self {
            lens: Vec::new(),
            lens_values: HashMap::new(),
        }
    }

    fn add_len(&mut self, label: String, focal_value: u8) {
        if !self.lens_values.contains_key(&label) {
            self.lens.push(label.clone());
        }
        self.lens_values.insert(label, focal_value);
    }

    fn remove_len(&mut self, label: &str) {
        if self.lens_values.remove(label).is_some() {
            if let Some(i) = self.lens.iter().position(|l| l == label) {
                self.lens.remove(i);
            }
        }
    }

    fn focusing_strength(&self) -> u32 {
        self.lens
            .iter()
            .enumerate()
            .map(|(pos, l)| *self.lens_values.get(l).unwrap() as u32 * (pos as u32 + 1))
            .sum()
    }
}

enum Operation {
    Add(String, u8),
    Remove(String),
}

impl FromStr for Operation {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("-") {
            Ok(Operation::Remove(s[0..s.len() - 1].to_string()))
        } else {
            let mut parts = s.split('=');
            let label = parts.next().context("No label")?.to_string();
            let value = parts.next().context("No focus value")?.parse()?;
            Ok(Operation::Add(label, value))
        }
    }
}

#[derive(Debug)]
struct Line {
    cartons: Vec<Carton>,
}

impl Line {
    fn new() -> Self {
        let mut cartons = Vec::new();
        for _ in 0..256 {
            cartons.push(Carton::new());
        }
        Self { cartons }
    }

    fn apply_operation(&mut self, operation: Operation) {
        match operation {
            Operation::Add(label, focus_value) => {
                let pos = hash(&label) as usize;
                self.cartons[pos].add_len(label, focus_value);
            }
            Operation::Remove(label) => {
                let pos = hash(&label) as usize;
                self.cartons[pos].remove_len(&label);
            }
        }
    }

    fn focusing_strength(&self) -> u32 {
        self.cartons
            .iter()
            .enumerate()
            .map(|(idx, c)| c.focusing_strength() * (idx as u32 + 1))
            .sum()
    }
}

pub fn fifteenth_task_2(f: impl BufRead) -> u64 {
    let mut line = Line::new();
    let s = f.lines().next().unwrap().unwrap();
    s.split(',')
        .map(|s| s.parse::<Operation>().unwrap())
        .for_each(|op| line.apply_operation(op));
    println!("line: {:?}", line);
    line.focusing_strength() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }
}
