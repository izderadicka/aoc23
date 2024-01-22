use std::{
    collections::{hash_map::RandomState, HashMap, HashSet},
    io::BufRead,
};

pub fn eleventh_task_2(f: impl BufRead) -> u64 {
    unsafe {
        EXPANSION = 1_000_000;
    }
    eleventh_task_1(f)
}

pub fn eleventh_task_1(f: impl BufRead) -> u64 {
    let mut stars: Vec<(i64, i64)> = Vec::new();
    for (r, line) in f.lines().enumerate() {
        let line = line.unwrap();
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                stars.push((r as i64, c as i64));
            }
        }
    }
    println!("stars: {:?}", stars);
    let occupied_rows: HashSet<_, RandomState> = HashSet::from_iter(stars.iter().map(|&(r, _)| r));
    let occupied_cols: HashSet<_, RandomState> = HashSet::from_iter(stars.iter().map(|&(_, c)| c));
    let mut occupied_rows = occupied_rows.into_iter().collect::<Vec<_>>();
    let mut occupied_cols = occupied_cols.into_iter().collect::<Vec<_>>();
    occupied_rows.sort();
    occupied_cols.sort();

    println!("occupied rows: {:?}", occupied_rows);
    println!("occupied cols: {:?}", occupied_cols);

    let row_scale = construct_scale(&occupied_rows);
    let col_scale = construct_scale(&occupied_cols);

    println!("row scale: {:?}", row_scale);
    println!("col scale: {:?}", col_scale);

    let num_stars = stars.len();
    let mut sum = 0;
    for i in 0..num_stars - 1 {
        for j in i + 1..num_stars {
            let pos1 = stars[i];
            let pos2 = stars[j];
            let dist = calculate_distance(pos1, pos2, &row_scale, &col_scale);

            sum += dist;
            // println!("{} - {} = {}", pos1, pos2, dist);
        }
    }

    sum
}

fn calculate_distance(
    (r1, c1): (i64, i64),
    (r2, c2): (i64, i64),
    row_scale: &HashMap<i64, i64>,
    col_scale: &HashMap<i64, i64>,
) -> u64 {
    let (r1, c1) = (row_scale[&r1], col_scale[&c1]);
    let (r2, c2) = (row_scale[&r2], col_scale[&c2]);
    let dist = (r1 - r2).abs() + (c1 - c2).abs();
    dist as u64
}

static mut EXPANSION: i64 = 2;

fn construct_scale(occupied: &[i64]) -> HashMap<i64, i64> {
    let mut previous = -1;
    let mut offset = 0;
    let mut scale = HashMap::with_capacity(occupied.len());
    let expansion = unsafe { EXPANSION };
    for n in occupied {
        let slot = (n - previous - 1);
        let slot = slot * expansion - slot;
        offset = offset + slot;
        scale.insert(*n, n + offset);
        previous = *n;
    }
    scale
}
