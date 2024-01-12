use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    num::ParseIntError,
    str::FromStr,
    vec,
};

#[derive(Debug, Default)]
struct Card {
    pub id: u32,
    wins: HashSet<u32>,
    tips: HashSet<u32>,
}

impl Card {
    fn number_of_matches(&self) -> u32 {
        self.tips.intersection(&self.wins).count() as u32
    }
}
lazy_static! {
    static ref CARD_REGEX: Regex = Regex::new(r"Card\s+(\d+): (.+) \| (.+)").unwrap();
}

fn parse_nums(nums: &str) -> Result<HashSet<u32>, ParseIntError> {
    nums.split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse())
        .collect()
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = CARD_REGEX.captures(s).unwrap();
        let id = captures.get(1).unwrap().as_str().parse()?;
        let wins = parse_nums(captures.get(2).unwrap().as_str())?;

        let tips = parse_nums(captures.get(3).unwrap().as_str())?;
        Ok(Self { id, wins, tips })
    }
}

pub fn fourth_task_1(f: impl BufRead) -> u32 {
    let mut sum = 0;
    for line in f.lines() {
        let line = line.unwrap();
        let card = line.parse::<Card>().unwrap();
        let matches = card.number_of_matches();
        println!("{} matches: {}", line, matches);
        if matches > 0 {
            let score = 2u32.pow(matches - 1);
            sum += score;
        }
    }

    sum
}

pub fn fourth_task_2(f: impl BufRead) -> u32 {
    let mut scores = vec![];
    for line in f.lines() {
        let line = line.unwrap();
        let card = line.parse::<Card>().unwrap();
        let matches = card.number_of_matches();
        scores.push(matches);
    }

    println!("{:?}", scores);
    let size = scores.len();
    let mut won_cards = vec![0; size];
    for (n, score) in scores.into_iter().enumerate() {
        let copies = won_cards[n];
        for next in n + 1..(n + score as usize + 1).min(size + 1) {
            //println!("{} -> {}", n, next);
            won_cards[next] += 1 + copies;
        }
    }
    println!("{:?}", won_cards);

    won_cards.into_iter().sum::<u32>() + size as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cards() {
        let input = "Card 1: 1 2 3 | 3 4 5 6";
        let card = input.parse::<Card>().unwrap();
        assert_eq!(card.id, 1);
        assert_eq!(card.wins.len(), 3);
        assert_eq!(card.tips.len(), 4);
        assert_eq!(card.number_of_matches(), 1);
    }
}
