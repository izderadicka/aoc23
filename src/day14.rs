use std::{fmt::Display, io::BufRead};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
    Empty,
    RoundedRock,
    SquareRock,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::RoundedRock => write!(f, "O"),
            Cell::SquareRock => write!(f, "#"),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Empty),
            'O' => Ok(Cell::RoundedRock),
            '#' => Ok(Cell::SquareRock),
            _ => anyhow::bail!("Unknown cell: {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn all() -> [Self; 4] {
        use Direction::*;
        [North, West, South, East]
    }
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn parse<R: BufRead>(reader: R) -> Self {
        let mut map = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let row = line
                .chars()
                .map(|c| c.try_into().unwrap())
                .collect::<Vec<_>>();
            map.push(row);
        }
        let width = map[0].len();
        let height = map.len();
        Self { map, width, height }
    }

    fn roll(&mut self, i: usize, j: usize, spaces: &mut Vec<Option<usize>>, increment: isize) {
        match self.map[i][j] {
            Cell::Empty => match spaces[j] {
                Some(_n) => {}
                None => spaces[j] = Some(i),
            },
            Cell::RoundedRock => match spaces[j] {
                Some(n) => {
                    self.map[n][j] = Cell::RoundedRock;
                    self.map[i][j] = Cell::Empty;
                    //let new_n = ((n as isize) + increment).max(0) as usize;
                    let new_n = n + 1;
                    spaces[j] = Some(new_n);
                }
                None => {}
            },
            Cell::SquareRock => spaces[j] = None,
        }
    }

    fn slide(&mut self, direction: Direction) {
        use Direction::*;
        let sz = match direction {
            North | South => self.width,
            East | West => self.height,
        };

        let mut spaces: Vec<Option<usize>> = vec![None; sz];
        match direction {
            North => {
                for i in 0..self.map.len() {
                    for j in 0..self.width {
                        self.roll(i, j, &mut spaces, 1);
                    }
                }
            }
            West => todo!(),
            South => {
                for i in (0..self.map.len()).rev() {
                    for j in 0..self.width {
                        self.roll(i, j, &mut spaces, -1);
                    }
                }
            }
            East => todo!(),
        }
        for i in 0..self.map.len() {
            for j in 0..self.width {
                self.roll(i, j, &mut spaces, 1);
            }
        }
    }

    fn north_weight(&self) -> u64 {
        let sz = self.map.len();
        let mut sum = 0;
        for (i, row) in self.map.iter().enumerate() {
            let weight = sz - i;
            let num_rounded = row.iter().filter(|&&c| c == Cell::RoundedRock).count();

            sum += (weight * num_rounded) as u64;
        }
        sum
    }
}

pub fn fourteens_task_1(f: impl BufRead) -> u64 {
    let mut map = Map::parse(f);
    println!("before:\n{}", map);
    map.slide(Direction::North);
    println!("after:\n{}", map);
    map.north_weight()
}
