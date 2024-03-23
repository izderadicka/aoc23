use anyhow::Context;
use colored::{ColoredString, Colorize};
use std::{io::BufRead, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }

    fn jump(&self, pos: (i32, i32), steps: i32) -> (i32, i32) {
        (
            pos.0 + steps * self.offset().0,
            pos.1 + steps * self.offset().1,
        )
    }

    fn go<F>(&self, pos: (i32, i32), steps: i32, mut f: F) -> (i32, i32)
    where
        F: FnMut(i32, i32),
    {
        let (mut row, mut col) = pos;
        for _ in 0..steps {
            f(row, col);
            (row, col) = self.jump((row, col), 1);
        }
        (row, col)
    }
}

impl FromStr for Direction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => anyhow::bail!("Unknown direction: {}", s),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Instruction {
    direction: Direction,
    steps: i32,
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
        let direction = parts
            .next()
            .context("No direction")?
            .parse()
            .context("Invalid direction")?;
        let steps = parts.next().context("No steps")?.parse()?;
        let color = parts.next().context("No color")?.to_string();
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
    let mut min_row = i32::MAX;
    let mut max_row = i32::MIN;
    let mut min_col = i32::MAX;
    let mut max_col = i32::MIN;
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

    let mut map = vec![vec![None; width as usize]; height as usize];
    let mut row = row_offset;
    let mut col = col_offset;

    for instruction in &instructions {
        let draw = |row: i32, col: i32| {
            map[row as usize][col as usize] = Some(Cell::Trench);
        };
        (row, col) = instruction
            .direction
            .go((row, col), instruction.steps, draw);
    }

    let mut size = 0;

    for row in 0..height as usize {
        let mut inside = false;
        let mut prev_cell = None;
        let mut in_row = 0;
        for col in 0..width as usize {
            let cell = map[row][col].clone();
            if cell.is_some() {
                size += 1;
                if let Some(Cell::Trench) = prev_cell {
                    in_row += 1;
                } else {
                    in_row = 1;
                }
            } else {
                let mut is_peak = false;
                if in_row > 1 {
                    if row == 0 || row == height as usize - 1 {
                        is_peak = true;
                    } else {
                        let start = map[row + 1][col-in_row].as_ref();
                        let end = map[row + 1][col-1].as_ref();

                        // println!("({}, {}): ({:?}, {:?})", row, col, start, end);

                        if (start.is_some() && end.is_some()) || (start.is_none() && end.is_none()) {
                            is_peak = true;
                        }
                        
                    }
                }

                if in_row > 0 && !is_peak {
                    inside = !inside
                }

                if inside {
                    size += 1;
                    map[row][col] = Some(Cell::Basin);
                };
                is_peak = false;
                in_row = 0;
            }

            prev_cell = cell.clone();
        }
    }

    print_map(&map);
    println!();
    size
}

fn print_map(map: &Vec<Vec<Option<Cell>>>) {
    for row in map {
        for cell in row {
            if let Some(c) = cell {
                match c {
                    Cell::Trench => print!("{}", "@".red()),
                    Cell::Basin => print!("#"),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}
