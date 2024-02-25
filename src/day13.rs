use std::io::BufRead;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
    Ash,
    Rock,
}

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Ash),
            '#' => Ok(Cell::Rock),
            _ => anyhow::bail!("Unknown cell: {}", value),
        }
    }
}

#[derive(Debug)]
struct Mirror {
    map: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Mirror {
    fn parse<R: BufRead>(reader: &mut R) -> Option<Self> {
        let mut map = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            if line.is_empty() {
                break;
            }
            let row = line
                .chars()
                .map(|c| c.try_into().unwrap())
                .collect::<Vec<_>>();
            map.push(row);
        }

        if map.is_empty() {
            return None;
        }

        let width = map[0].len();
        let height = map.len();
        Some(Mirror { map, width, height })
    }

    fn transpose(self) -> Self {
        let mut map = Vec::new();
        for i in 0..self.width {
            let mut row = Vec::new();
            for j in 0..self.height {
                row.push(self.map[j][i]);
            }
            map.push(row);
        }
        Mirror {
            map,
            width: self.height,
            height: self.width,
        }
    }

    fn split(&self) -> Option<usize> {
        'outer: for i in 1..self.map.len() {
            let mut fixed = false;
            let mut can_fix = |v1, v2| {
                if !fixed && distance(v1, v2) == 1 {
                    // println!("Fixing {:?} and {:?}", v1, v2);
                    fixed = true;
                    true
                } else {
                    false
                }
            };
            if self.map[i] == self.map[i - 1] || can_fix(&self.map[i], &self.map[i - 1]) {
                let steps = (self.map.len() - i - 1).min(i - 1);
                for j in 1..=steps {
                    if can_fix(&self.map[i + j], &self.map[i - j - 1]) {
                        continue;
                    }
                    if self.map[i + j] != self.map[i - j - 1] {
                        continue 'outer;
                    }
                }
                if fixed {
                    return Some(i);
                }
            }
        }
        None
    }
}

fn distance(v1: &Vec<Cell>, v2: &Vec<Cell>) -> usize {
    v1.iter()
        .zip(v2.iter())
        .map(|(a, b)| if a == b { 0 } else { 1 })
        .sum()
}

pub fn thirteens_task_2(mut f: impl BufRead) -> u64 {
    let mut nun_mirrors = 0;
    let mut sum = 0;
    loop {
        if let Some(mirror) = Mirror::parse(&mut f) {
            let mut has_split = false;
            if let Some(split) = mirror.split() {
                println!("Mirror {} horizontal split at {}", nun_mirrors, split);
                sum += split as u64 * 100;
                has_split = true;
            }

            if let Some(split) = mirror.transpose().split() {
                println!("Mirror {} vertical split at {}", nun_mirrors, split);
                sum += split as u64;
                has_split = true;
            }

            if !has_split {
                panic!("No split found in mirror {}", nun_mirrors);
            }

            nun_mirrors += 1;
        } else {
            break;
        }
    }
    sum
}
