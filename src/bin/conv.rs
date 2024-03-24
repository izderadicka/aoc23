use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

fn main() {
    // open file and read
    let file_name = env::args().nth(1).expect("Missing file name");
    let f = BufReader::new(File::open(file_name).expect("Problem opening file"));

    let mut out = BufWriter::new(
        File::create(env::args().nth(2).expect("Missing output file name"))
            .expect("Problem creating file"),
    );

    for line in f.lines() {
        let line = line.expect("Problem reading line");
        let mut items = line.split_whitespace();
        let (dir, steps, _color) = (
            items.next().unwrap(),
            items.next().unwrap().parse::<u32>().unwrap(),
            items.next().unwrap(),
        );

        let dir_num = match dir {
            "R" => 0,
            "D" => 1,
            "L" => 2,
            "U" => 3,
            _ => panic!("Bad direction"),
        };

        let new_color = dir_num + 16 * steps;

        let new_line = format!("{} {} (#{:06x})", dir, steps, new_color);

        out.write_all(new_line.as_bytes())
            .expect("Problem writing line");
        out.write_all("\n".as_bytes())
            .expect("Problem writing line");
    }
}
