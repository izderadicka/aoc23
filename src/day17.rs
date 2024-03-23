use std::{
    cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashMap, HashSet}, io::BufRead
};

use colored::{ColoredString, Colorize};

trait Limit {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Limit for Map {
    fn width(&self) -> usize {
        self.width  
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl Map {
    fn parse<R: BufRead>(reader: R) -> Self {
        let mut map = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            map.push(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>(),
            );
        }
        let width = map[0].len();
        let height = map.len();
        Map { map, width, height }
    }

    fn print_path(&self, path: impl Iterator<Item=Position>) {
        let mut visited = HashSet::new();
        visited.extend(path);
        for row in 0..self.height {
            for col in 0..self.width {
                let mut n: ColoredString = self.map[row][col].to_string().into();
                if visited.contains(&Position{row, col}) {
                    n = n.red();
                }
                print!("{}", n)
            }
            println!("")
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Course {
    direction: Direction,
    within_line: u32,
}

impl Course {
    fn go<T:Limit>(&self, pos: &Position, new_dir: Direction, limits: &T, min_line:u32, max_line: u32) -> Option<(Position, Self)> {

        let mut row = pos.row as isize;
        let mut col = pos.col as isize;

        match new_dir {
            Direction::Left => col-=1,
            Direction::Right => col+=1,
            Direction::Up => row-=1,
            Direction::Down => row+=1,
        }


        if col < 0 || col >= limits.width() as isize || row < 0 || row >= limits.height() as isize {
            return None;
        }

        if new_dir == self.direction && self.within_line >= max_line {
            return None;
        }

       

        if new_dir != self.direction && self.within_line < min_line {
            return None;
        }

        let within_line = if new_dir == self.direction { self.within_line + 1} else {1};
        
        Some((
            Position {
                row: row as usize,
                col: col as usize,
            },
            Course {
                direction: new_dir,
                within_line
            }
        )
        )
    }
}


#[derive(PartialEq,Eq, Clone, Debug)]
struct PositionWithState {
    len: u32, 
    pos: Position,
    course: Course,
     path: Vec<(Position, u32)>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Visited {
    pos: Position,
    course: Course
}

impl PartialOrd for PositionWithState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut res =other.len.partial_cmp(&self.len);
        if let  Some(Ordering::Equal) = res{
            res = other.pos.partial_cmp(&self.pos)
            
        } 
        res
}
}

impl Ord for PositionWithState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}


fn find_len(map: &Map, min_line:u32, max_line: u32) -> Option<(u32, Vec<(Position, u32)>)> {
    let mut remaining: BinaryHeap<PositionWithState> = BinaryHeap::new();
    let mut visited: HashSet<Visited> = HashSet::new(); 
    let mut iteration = 0;

    for dir in [Direction::Right, Direction::Down] {
        remaining.push(
            PositionWithState {
            pos:Position { row: 0, col: 0 },
            course: Course {
                direction: dir,
                within_line: 1,},
                len: 0,
                path: vec![],
            },
        
        );
    }
    while let Some(PositionWithState{pos, course: prev_course, len:prev_len, path: prev_path}) = remaining.pop() {
        let v = Visited{ pos, course: prev_course.clone() };
        if visited.contains(&v) {
            continue;
        }
        // println!("Visited(len {}): {:?}", prev_len,v);
        visited.insert(v);
        
        

        iteration += 1;
        if iteration % 100000 == 0 {
            println!("Iteration: {}", iteration);
        }
        let mut  new_path =prev_path;
        new_path.push((pos, prev_len));

        if pos.row == map.height - 1 && pos.col == map.width - 1  && prev_course.within_line >= min_line {
            println!("Total iterations: {}", iteration);
            
            return Some((prev_len, new_path));
        }

        
        use Direction::*;
        for dir in [Right, Down, Left, Up,]
            .into_iter()
            .filter(|d| *d != prev_course.direction.opposite())
        {
            

            prev_course.go(&pos, dir.clone(), map, min_line, max_line).map(|(new_pos, new_course)| {
                let new_len = prev_len
                .saturating_add(map.map[new_pos.row][new_pos.col]);
                let v = Visited{ pos: new_pos, course: new_course};
                if ! visited.contains(&v) {
                    let next = PositionWithState{pos: v.pos, course: v.course, len: new_len, path: new_path.clone()};
                    // println!("Pushing: {:?}", next);
                remaining.push(next)
                } else {
                    // println!("Already visited: {:?}", v);
                }
            });
        }
    }

    None
}

// fn navigate(cache: &mut HashMap<State, u32>, map: &Map, pos: State, mut len: u32) -> u32 {
//     println!("Entering {:?}", pos);

//     if let Some(val) = cache.get(&pos) {
//         return *val;
//     }

//     // if len > 2000 {
//     //     return u32::MAX;
//     // }

//     if pos.position.row == map.height - 1 && pos.position.col == map.width - 1 {
//         let final_len = len.saturating_add(map.map[pos.position.row][pos.position.col]);
//         println!("Done in {} len", final_len);
//         return final_len;
//     }
//     use Direction::*;

//     fn create_state(
//         row: isize,
//         col: isize,
//         new_dir: Direction,
//         pos: &State,
//         map: &Map,
//     ) -> Option<State> {
//         if col < 0 || col >= map.width as isize || row < 0 || row >= map.height as isize {
//             return None;
//         }

//         if new_dir == pos.direction && pos.within_line >= 3 {
//             return None;
//         }

//         Some(State {
//             position: Position {
//                 row: row as usize,
//                 col: col as usize,
//             },
//             direction: new_dir,
//             within_line: if new_dir == pos.direction {
//                 pos.within_line + 1
//             } else {
//                 1
//             },
//         })
//     }

//     let mut next_pos = Vec::new();

//     for dir in [Left, Right, Up, Down]
//         .into_iter()
//         .filter(|d| d != &pos.direction.opposite())
//     {
//         let next = match dir {
//             Left => create_state(
//                 pos.position.row as isize,
//                 pos.position.col as isize - 1,
//                 dir,
//                 &pos,
//                 &map,
//             ),
//             Right => create_state(
//                 pos.position.row as isize,
//                 pos.position.col as isize + 1,
//                 dir,
//                 &pos,
//                 &map,
//             ),

//             Up => create_state(
//                 pos.position.row as isize - 1,
//                 pos.position.col as isize,
//                 dir,
//                 &pos,
//                 &map,
//             ),

//             Down => create_state(
//                 pos.position.row as isize + 1,
//                 pos.position.col as isize,
//                 dir,
//                 &pos,
//                 &map,
//             ),
//         };

//         next.and_then(|n| {
//             if cache.contains_key(&n) {
//                 println!("Skipping {:?}", n);
//                 None
//             } else {
//                 Some(n)
//             }
//         })
//         .map(|n| next_pos.push(n));
//     }

//     if !cache.is_empty() {
//         len += map.map[pos.position.row][pos.position.col];
//     }
//     cache.insert(pos.clone(), u32::MAX);

//     let mut new_len = u32::MAX;
//     for next in next_pos {
//         new_len = new_len.min(navigate(cache, map, next, len));
//     }
//     println!("Leaving {:?} {} -> {}", pos, len, new_len);

//     cache.insert(pos, new_len);
//     new_len
// }

pub fn seventeenth_task_1(f: impl BufRead) -> u32 {
    let map = Map::parse(f);
    println!("Map: {:?}", map);
    let (len, path) = find_len(&map, 4, 10).expect("No path found");
    println!("Path: {:?}\n", path);
    map.print_path(path.into_iter().map(|(p, _)| p));
    len
}
