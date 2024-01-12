use std::{collections::HashMap, io::BufRead};

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Number(u32),
    NumberAdjacent(u32),
    Symbol,
    Dot,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Number {
    pos: Pos,
    len: u32,
    number: u32,
}

pub fn third_task_2(f: impl BufRead) -> u32 {
    let mut numbers = Vec::new();
    let mut stars: HashMap<Pos, Vec<u32>> = HashMap::new();
    for (row, line) in f.lines().enumerate() {
        let line = line.expect("Problem reading line");
        let mut number = 0;
        let mut len = 0;
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                number = number * 10 + c.to_digit(10).unwrap();
                len += 1;
            } else {
                if len >= 1 {
                    numbers.push(Number {
                        pos: Pos {
                            row,
                            col: col - len as usize,
                        },
                        len,
                        number,
                    });
                }
                number = 0;
                len = 0;
                if c == '*' {
                    stars.insert(Pos { row, col }, Vec::new());
                }
            }
        }

        if len >= 1 {
            numbers.push(Number {
                pos: Pos {
                    row,
                    col: line.len() - len as usize,
                },
                len,
                number,
            });
        }
    }

    println!("Numbers: {:?}", numbers);

    for number in numbers {
        for (pos, items) in stars.iter_mut() {
            let col_start = number.pos.col.saturating_sub(1);
            let col_end = (number.pos.col + number.len as usize);
            let row_start = number.pos.row.saturating_sub(1);
            let row_end = (number.pos.row + 1);

            if pos.col >= col_start
                && pos.col <= col_end
                && pos.row >= row_start
                && pos.row <= row_end
            {
                items.push(number.number);
            }
        }
    }

    println!("Stars: {:?}", stars);

    stars
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum()
}

pub fn third_task_1(f: impl BufRead) -> u32 {
    let mut matrix = Vec::new();
    for line in f.lines() {
        let line = line.expect("Problem reading line");
        let cells: Vec<Cell> = line
            .chars()
            .map(|c| match c {
                '.' => Cell::Dot,
                n if n.is_ascii_digit() => Cell::Number(n.to_digit(10).unwrap()),
                _ => Cell::Symbol,
            })
            .collect();
        matrix.push(cells);
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    println!("Matrix rows: {} cols: {}", rows, cols);
    // mark adjacent numbers
    for row in 0..rows {
        for col in 0..cols {
            let cell = &matrix[row][col];
            if let Cell::Symbol = cell {
                let row_from = row.saturating_sub(1);
                let row_to = (row + 2).min(rows);
                let col_from = col.saturating_sub(1);
                let col_to = (col + 2).min(cols);
                for r in row_from..row_to {
                    for c in col_from..col_to {
                        if let Cell::Number(n) = &matrix[r][c] {
                            matrix[r][c] = Cell::NumberAdjacent(*n);
                        }
                    }
                }
            }
        }
    }
    // println!("Matrix: {:?}", matrix);
    //collect numbers
    let mut numbers: Vec<u32> = Vec::new();

    let mut current_number = 0;
    let mut is_adjascent = false;

    for row in 0..rows {
        current_number = 0;
        is_adjascent = false;
        for col in 0..matrix[row].len() {
            match &matrix[row][col] {
                cell @ (Cell::NumberAdjacent(n) | Cell::Number(n)) => {
                    current_number = current_number * 10 + n;
                    if let Cell::NumberAdjacent(_) = cell {
                        is_adjascent = true;
                    }
                }
                _ => {
                    if is_adjascent {
                        numbers.push(current_number);
                    }
                    current_number = 0;
                    is_adjascent = false;
                }
            }
        }
        if current_number > 0 && is_adjascent {
            numbers.push(current_number);
        }
    }
    println!("Numbers: {:?}", numbers);

    numbers.iter().sum()
}
