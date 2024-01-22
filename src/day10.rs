use std::{fmt::Display, io::BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    /// Movement offset in row, column
    fn offset(&self) -> (i64, i64) {
        match self {
            Direction::N => (-1, 0),
            Direction::S => (1, 0),
            Direction::E => (0, 1),
            Direction::W => (0, -1),
        }
    }

    fn inverse(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
        }
    }
}
#[derive(Debug)]
struct TrackedCell {
    cell: Cell,
    visited: bool,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty, // .
    N_S,   // |
    E_W,   // -
    N_E,   // L
    N_W,   // J
    S_W,   // 7
    S_E,   // F
    Start, // S
}

impl Cell {
    fn leads_north(&self) -> bool {
        matches!(self, Cell::N_S | Cell::N_E | Cell::N_W)
    }
}

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Empty),
            '|' => Ok(Cell::N_S),
            '-' => Ok(Cell::E_W),
            'L' => Ok(Cell::N_E),
            'J' => Ok(Cell::N_W),
            '7' => Ok(Cell::S_W),
            'F' => Ok(Cell::S_E),
            'S' => Ok(Cell::Start),
            _ => anyhow::bail!("Invalid character: {}", value),
        }
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<TrackedCell>>,
    width: usize,
    height: usize,
    start: (usize, usize),
}

impl Map {
    fn parse<R: BufRead>(reader: R) -> Self {
        let mut map = Vec::new();
        let mut row_no = 0;
        let mut start: Option<(usize, usize)> = None;
        for line in reader.lines() {
            let line = line.unwrap();

            map.push(
                line.chars()
                    .map(|c| c.try_into().unwrap())
                    .enumerate()
                    .map(|(i, c)| {
                        if c == Cell::Start {
                            start = Some((row_no, i));
                        }
                        TrackedCell {
                            cell: c,
                            visited: false,
                        }
                    })
                    .collect::<Vec<TrackedCell>>(),
            );
            row_no += 1;
        }

        let width = map[0].len();
        let height = map.len();
        let start = start.unwrap();
        Map {
            map,
            width,
            height,
            start,
        }
    }

    fn starting_points(&mut self) -> (Route, Route) {
        let mut res = vec![];
        self.map[self.start.0][self.start.1].visited = true;
        for dir in [Direction::N, Direction::S, Direction::E, Direction::W] {
            if let Some(pos) = self.move_to(self.start, dir) {
                if let Some(next_direction) = self.map[pos.0][pos.1].cell.next_cell_direction(dir) {
                    self.map[pos.0][pos.1].visited = true;
                    {
                        res.push({
                            Route {
                                len: 1,
                                next_direction,
                                pos,
                            }
                        })
                    }
                }
            }
        }
        assert!(
            res.len() == 2,
            "Expected exactly 2 starting points, got {}",
            res.len()
        );
        let mut iter = res.into_iter();
        (iter.next().unwrap(), iter.next().unwrap())
    }

    fn move_to(&self, pos: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
        let offset = dir.offset();
        let (row, col) = pos;
        let new_pos = (row as i64 + offset.0, col as i64 + offset.1);
        if new_pos.0 >= 0
            && new_pos.0 < self.height as i64
            && new_pos.1 >= 0
            && new_pos.1 < self.width as i64
        {
            Some((new_pos.0 as usize, new_pos.1 as usize))
        } else {
            None
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for cell in row {
                if cell.visited {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Cell {
    fn next_cell_direction(&self, from_other: Direction) -> Option<Direction> {
        let from = from_other.inverse();
        match (from, self) {
            (_, Cell::Empty | Cell::Start) => None,
            (Direction::N, Cell::N_S) => Some(Direction::S),
            (Direction::N, Cell::N_E) => Some(Direction::E),
            (Direction::N, Cell::N_W) => Some(Direction::W),
            (Direction::S, Cell::N_S) => Some(Direction::N),
            (Direction::S, Cell::S_W) => Some(Direction::W),
            (Direction::S, Cell::S_E) => Some(Direction::E),
            (Direction::E, Cell::N_E) => Some(Direction::N),
            (Direction::E, Cell::E_W) => Some(Direction::W),
            (Direction::E, Cell::S_E) => Some(Direction::S),
            (Direction::W, Cell::N_W) => Some(Direction::N),
            (Direction::W, Cell::E_W) => Some(Direction::E),
            (Direction::W, Cell::S_W) => Some(Direction::S),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Route {
    len: u64,
    next_direction: Direction,
    pos: (usize, usize),
}

impl Route {
    fn move_next(&mut self, map: &mut Map) {
        let new_pos = map.move_to(self.pos, self.next_direction).unwrap();
        let next_cell = &mut map.map[new_pos.0][new_pos.1];
        let old_direction = self.next_direction;
        self.next_direction = next_cell
            .cell
            .next_cell_direction(self.next_direction)
            .expect(&format!(
                "No route at {:?} towards {:?}",
                new_pos, self.next_direction,
            ));
        // println!(
        //     "moved from {:?} direction {:?} to {:?}, then can go {:?}",
        //     self.pos, old_direction, new_pos, self.next_direction
        // );
        self.pos = new_pos;
        self.len += 1;
        map.map[new_pos.0][new_pos.1].visited = true;
    }
}

pub fn tenth_task_2(f: impl BufRead) -> u64 {
    let mut map = Map::parse(f);
    println!("map: {:?}", map);
    let (mut r1, mut r2) = map.starting_points();
    println!("r1: {:?}, r2: {:?}", r1, r2);
    let max_depth = loop {
        r1.move_next(&mut map);
        if r1.pos == r2.pos {
            break r1.len.min(r2.len);
        }
        r2.move_next(&mut map);
        if r1.pos == r2.pos {
            break r1.len.min(r2.len);
        }
    };
    println!("map:\n{}", map);
    let mut inside_count = 0;
    for (r, row) in map.map.iter().enumerate() {
        let mut north_cells = 0;
        for (c, cell) in row.iter().enumerate() {
            if cell.visited {
                if cell.cell.leads_north() {
                    north_cells += 1
                };
            } else {
                if north_cells % 2 == 1 {
                    println!("INSIDE {} {}", r, c);
                    inside_count += 1;
                }
            }
        }
    }
    inside_count
}
