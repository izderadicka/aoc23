use anyhow::Context;
use std::{io::BufRead, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn offset(&self) -> (i64, i64) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }

    fn jump(&self, pos: (i64, i64), steps: i64) -> (i64, i64) {
        (
            pos.0 + steps * self.offset().0,
            pos.1 + steps * self.offset().1,
        )
    }
}

impl FromStr for Direction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Direction::Left),
            "0" => Ok(Direction::Right),
            "3" => Ok(Direction::Up),
            "1" => Ok(Direction::Down),
            _ => anyhow::bail!("Unknown direction: {}", s),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Instruction {
    direction: Direction,
    steps: i64,
    color: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Cell {
    Trench,
    Basin,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let _direction = parts.next().context("No direction")?;
        let _steps = parts.next().context("No steps")?;
        let color = parts.next().context("No color")?.to_string();
        let direction: Direction = (&color[7..8]).parse()?;
        let steps = i64::from_str_radix(&color[2..7], 16)?;
        Ok(Self {
            direction,
            steps,
            color,
        })
    }
}

pub fn area(poly: &Vec<(i64, i64)>) -> i64 {
    let mut sum = 0;
    for i in 0..poly.len() - 1 {
        // sum += (poly[i].0 + poly[i + 1].0) * (poly[i].1 - poly[i + 1].1)
        sum += poly[i].1 * poly[i + 1].0 - poly[i + 1].1 * poly[i].0
    }
    sum.abs() / 2
}

pub fn eighteens_task_1(f: impl BufRead) -> u64 {
    let instructions = f
        .lines()
        .map(|l| l.unwrap().parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();
    // println!("Instructions: {:?}", instructions);
    let (mut row, mut col) = (0, 0);
    let mut poly = vec![(row, col)];
    for instruction in &instructions {
        (row, col) = instruction.direction.jump((row, col), instruction.steps);
        poly.push((row, col));
    }

    println!("Poly {:?}", poly);

    let area = area(&poly);
    let cir: i64 = instructions.iter().map(|i| i.steps).sum();

    (area + cir / 2 + 1) as u64
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_area() {
        let p = vec![(0, 0), (0, 1), (1, 1), (1, 0), (0, 0)];
        let a = area(&p);
        assert_eq!(1, a);
    }
}
