use std::{collections::VecDeque, io::BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Working,
    Damaged,
    Unknown,
}

enum Next {
    Some(Vec<Sample>),
    None(Sample),
}

impl TryFrom<char> for Token {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Token::Working),
            '#' => Ok(Token::Damaged),
            '?' => Ok(Token::Unknown),
            _ => anyhow::bail!("Unknown token: {}", value),
        }
    }
}
#[derive(Debug, Clone)]
struct Sample {
    remaining_pattern: VecDeque<Token>,
    groups: Vec<u16>,
    in_group: bool,
}
impl Sample {
    fn new(pattern: &str) -> Self {
        Self {
            remaining_pattern: pattern.chars().map(|x| x.try_into().unwrap()).collect(),
            groups: Vec::new(),
            in_group: false,
        }
    }

    fn inc_group(&mut self) {
        if self.in_group {
            self.groups.last_mut().map(|x| *x += 1).expect("no group");
        } else {
            self.groups.push(1);
        }
        self.in_group = true
    }

    fn to_next(mut self) -> Next {
        match self.remaining_pattern.pop_front() {
            Some(Token::Working) => {
                self.in_group = false;
                Next::Some(vec![self])
            }
            Some(Token::Damaged) => {
                self.inc_group();
                Next::Some(vec![self])
            }
            Some(Token::Unknown) => {
                let mut first = self.clone();
                first.inc_group();
                let mut second = self;
                second.in_group = false;
                Next::Some(vec![first, second])
            }
            None => Next::None(self),
        }
    }

    fn is_valid(&self, groups: &[u16]) -> bool {
        self.remaining_pattern.is_empty() && self.groups == groups
    }

    fn is_acceptable(&self, groups: &[u16]) -> bool {
        self.groups.len() <= groups.len()
            && self.groups.iter().zip(groups.iter()).all(|(x, y)| x <= y)
    }

    fn is_complete(&self) -> bool {
        self.remaining_pattern.is_empty()
    }
}

fn process_one(pattern: &str, groups: &[u16]) -> u64 {
    let mut samples = vec![Sample::new(pattern)];
    let mut count = 0;
    while !samples.is_empty() {
        let sample = samples.pop().unwrap();
        match sample.to_next() {
            Next::Some(new_samples) => {
                samples.extend(new_samples.into_iter().filter(|x| x.is_acceptable(groups)))
            }
            Next::None(sample) => {
                if sample.is_valid(groups) {
                    count += 1
                }
            }
        }
    }
    count
}

pub fn twelveth_task_1(f: impl BufRead) -> u64 {
    let mut sum = 0;
    for line in f.lines() {
        let line = line.unwrap();
        let mut iter = line.split_ascii_whitespace();
        let pattern = iter.next().unwrap();
        let groups: Vec<_> = iter
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u16>().unwrap())
            .collect();
        let variants = process_one(pattern, &groups);
        println!("{} {:?} => {}", pattern, groups, variants);
        sum += variants;
    }

    sum
}
