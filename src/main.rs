#![allow(dead_code)]
use std::fs::File;
use std::io::BufReader;

use crate::day19::nineteenth_task_2 as the_task;

mod day19;
fn main() {
    let file_name = std::env::args().nth(1).expect("Missing file name");
    let f = BufReader::new(File::open(file_name).expect("Problem opening file"));
    let res = the_task(f);
    println!("Result: {}", res);
}

