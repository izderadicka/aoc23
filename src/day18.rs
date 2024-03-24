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

    fn go<F>(&self, pos: (i64, i64), steps: i64, mut f: F) -> (i64, i64)
    where
        F: FnMut(i64, i64, i64),
    {
        let (mut row, mut col) = pos;
        match self {
            Direction::Left => {
                f(row, col - steps, steps);
                (row, col) = (row, col - steps);
            }
            Direction::Right => {
                f(row, col, steps);
                (row, col) = (row, col + steps);
            }
            Direction::Up => {
                for _ in 0..steps {
                    (row, col) = (row - 1, col);
                    f(row, col, 1);
                }
            }
            Direction::Down => {
                for _ in 0..steps {
                    (row, col) = (row + 1, col);
                    f(row, col, 1);
                }
            }
        }

        (row, col)
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

pub fn eighteens_task_1(f: impl BufRead) -> u64 {
    let instructions = f
        .lines()
        .map(|l| l.unwrap().parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();
    println!("Instructions: {:?}", instructions);
    let mut min_row = i64::MAX;
    let mut max_row = i64::MIN;
    let mut min_col = i64::MAX;
    let mut max_col = i64::MIN;
    let mut row = 0;
    let mut col = 0;
    for instruction in &instructions {
        (row, col) = instruction.direction.jump((row, col), instruction.steps);
        min_row = min_row.min(row);
        max_row = max_row.max(row);
        min_col = min_col.min(col);
        max_col = max_col.max(col);
    }

    assert!(row == 0 && col == 0);

    let height = max_row - min_row + 1;
    let width = max_col - min_col + 1;
    let row_offset = -min_row;
    let col_offset = -min_col;

    println!("Height: {}, Width: {}", height, width);
    println!("Row offset: {}, Col offset: {}", row_offset, col_offset);
    println!();
    let mut map = vec![vec![]; height as usize];
    let mut row = row_offset;
    let mut col = col_offset;

    for instruction in &instructions {
        let draw = |row: i64, col: i64, steps: i64| {
            map[row as usize].push((col, steps));
        };
        let (new_row, _new_col) = instruction
            .direction
            .go((row, col), instruction.steps, draw);
        // println!("({},{})=>({},{})", row, col, new_row, _new_col);
        (row, col) = (new_row, _new_col);
    }

    let max_elems = map.iter().map(|row| row.len()).max().unwrap();
    map.iter_mut().for_each(|row| row.sort());
    println!("Max elems: {}", max_elems);

    let mut size = 0;
    println!("Map: {:?}", map);

    for row in 0..height as usize {
        let mut inside = false;
        let mut prev_col = -1;
        let mut span = 0;
        for (col, in_row) in map[row].iter() {
            let span_incr = col - prev_col;
            if *col == prev_col {
                span += span_incr;
            } else {
                span = span_incr;
            }
            size += *in_row as u64;
            if inside {
                size += span as u64;
            }

            let mut is_peak = false;
            if row == 0 || row == height as usize - 1 {
                is_peak = true;
            } else {
                let next_row = &map[row + 1];

                let start = next_row.iter().find(|(c, _)| *c == *col - span);
                let end = next_row.iter().find(|(c, _)| *c == *col + *in_row - 1);
                let (has_start, has_end) = (start.is_some(), end.is_some());

                // println!("({}, {}): ({:?}, {:?})", row, col, start, end);

                if (has_start && has_end) || (!has_start && !has_end) {
                    is_peak = true;
                }
            }

            if !is_peak && span > 0 {
                println!("Swap inside at ({}, {})", row, col);
                inside = !inside;
            }

            prev_col = *col + *in_row;
        }
    }

    println!();
    size
}
