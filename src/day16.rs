use std::{collections::HashSet, io::BufRead, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum MirrorType {
    RightDown,
    RightUp,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum PipeType {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn offset(&self) -> Vec<(i32, i32, Direction)> {
        match self {
            Direction::Up => vec![(-1, 0, *self)],
            Direction::Down => vec![(1, 0, *self)],
            Direction::Left => vec![(0, -1, *self)],
            Direction::Right => vec![(0, 1, *self)],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
    Empty,
    Mirror(MirrorType),
    Pipe(PipeType),
}

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;
    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '.' => Ok(Cell::Empty),
            '|' => Ok(Cell::Pipe(PipeType::Vertical)),
            '-' => Ok(Cell::Pipe(PipeType::Horizontal)),
            '/' => Ok(Cell::Mirror(MirrorType::RightUp)),
            '\\' => Ok(Cell::Mirror(MirrorType::RightDown)),
            _ => anyhow::bail!("Unknown cell: {}", s),
        }
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
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
        Map {
            width: map[0].len(),
            height: map.len(),
            map,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct PathElement {
    direction: Direction,
    row: i32,
    col: i32,
}

impl PathElement {
    fn next(&self, map: &Map) -> Vec<PathElement> {
        use Cell::*;
        use Direction::*;
        use MirrorType::*;
        use PipeType::*;

        fn concat<T>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
            v1.into_iter().chain(v2.into_iter()).collect()
        }

        let offsets = match (
            self.direction,
            map.map[self.row as usize][self.col as usize],
        ) {
            (Up, Empty) => Up.offset(),
            (Up, Mirror(m)) => match m {
                RightDown => Left.offset(),
                RightUp => Right.offset(),
            },
            (Up, Pipe(p)) => match p {
                Horizontal => concat(Left.offset(), Right.offset()),
                Vertical => Up.offset(),
            },
            (Down, Empty) => Down.offset(),
            (Down, Mirror(m)) => match m {
                RightDown => Right.offset(),
                RightUp => Left.offset(),
            },
            (Down, Pipe(p)) => match p {
                Horizontal => concat(Left.offset(), Right.offset()),
                Vertical => Down.offset(),
            },
            (Left, Empty) => Left.offset(),
            (Left, Mirror(m)) => match m {
                RightDown => Up.offset(),
                RightUp => Down.offset(),
            },
            (Left, Pipe(p)) => match p {
                Vertical => concat(Up.offset(), Down.offset()),
                Horizontal => Left.offset(),
            },
            (Right, Empty) => Right.offset(),
            (Right, Mirror(m)) => match m {
                RightDown => Down.offset(),
                RightUp => Up.offset(),
            },
            (Right, Pipe(p)) => match p {
                Vertical => concat(Up.offset(), Down.offset()),
                Horizontal => Right.offset(),
            },
        };

        offsets
            .into_iter()
            .map(|(row_offset, col_offset, dir)| PathElement {
                direction: dir,
                row: self.row + row_offset,
                col: self.col + col_offset,
            })
            .filter(|p| {
                p.row >= 0 && p.row < map.height as i32 && p.col >= 0 && p.col < map.width as i32
            })
            .collect()
    }
}

struct Path {
    visited: HashSet<PathElement>,
    pending: Vec<PathElement>,
}

impl Path {
    fn new() -> Self {
        Path {
            visited: HashSet::new(),
            pending: vec![PathElement {
                direction: Direction::Right,
                row: 0,
                col: 0,
            }],
        }
    }

    fn new_from(p: PathElement) -> Self {
        Path {
            visited: HashSet::new(),
            pending: vec![p],
        }
    }

    fn eval(&mut self, map: &Map) {
        while let Some(element) = self.pending.pop() {
            if self.visited.contains(&element) {
                continue;
            }

            let next = element.next(map);
            self.pending
                .extend(next.into_iter().filter(|e| !self.visited.contains(e)));

            self.visited.insert(element);
        }
    }

    fn number_of_visited_cells(&self) -> u64 {
        let res: HashSet<_> = self.visited.iter().map(|p| (p.row, p.col)).collect();
        res.len() as u64
    }
}

pub fn sixteenth_task_1(f: impl BufRead) -> u64 {
    let map = Map::parse(f);
    let mut path = Path::new();
    println!("map: {:?}", map);
    path.eval(&map);

    path.number_of_visited_cells()
}

fn start_from(p: PathElement, map: &Map) -> u64 {
    let mut path = Path::new_from(p);
    path.pending.push(p);
    path.eval(&map);

    path.number_of_visited_cells()
}

pub fn sixteenth_task_2(f: impl BufRead) -> u64 {
    let map = Map::parse(f);
    let mut max = 0;
    for row in 0..map.height as i32 {
        for (start, direction) in [(0, Direction::Right), (map.width - 1, Direction::Left)] {
            let p = PathElement {
                direction,
                row,
                col: start as i32,
            };
            let res = start_from(p, &map);
            if res > max {
                max = res;
            }
        }
    }

    for col in 0..map.width as i32 {
        for (start, direction) in [(0, Direction::Down), (map.height - 1, Direction::Up)] {
            let p = PathElement {
                direction,
                row: start as i32,
                col,
            };
            let res = start_from(p, &map);
            if res > max {
                max = res;
            }
        }
    }

    max
}
