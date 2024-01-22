use std::io::BufRead;

pub fn nineth_task_1(f: impl BufRead) -> i64 {
    let mut sum = 0;
    for line in f.lines() {
        let line = line.unwrap();
        let row: Vec<_> = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        sum += estimate_next(row)
    }
    sum
}

pub fn nineth_task_2(f: impl BufRead) -> i64 {
    let mut sum = 0;
    for line in f.lines() {
        let line = line.unwrap();
        let row: Vec<_> = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        sum += estimate_previous(row)
    }
    sum
}

fn estimate_previous(mut row: Vec<i64>) -> i64 {
    println!("ROW: {:?}", row);
    let mut firsts = vec![];
    while !row.iter().all(|x| *x == 0) {
        firsts.push(*row.first().unwrap());
        let diffs = row.windows(2).map(|x| x[1] - x[0]).collect();
        println!("\t{:?}", diffs);
        row = diffs;
    }
    println!("FIRSTS: {:?}", firsts);
    firsts.into_iter().rfold(0, |acc, x| x - acc)
}

fn estimate_next(mut row: Vec<i64>) -> i64 {
    println!("ROW: {:?}", row);
    let mut lasts = vec![];
    while !row.iter().all(|x| *x == 0) {
        lasts.push(*row.last().unwrap());
        let diffs = row.windows(2).map(|x| x[1] - x[0]).collect();
        println!("\t{:?}", diffs);
        row = diffs;
    }
    println!("LASTS: {:?}", lasts);
    lasts.into_iter().sum()
}
