use std::io::BufRead;

fn parse_group(s: &str, expected_label: &str) -> Vec<u64> {
    let mut label_values = s.split(": ");
    let label = label_values.next().unwrap();
    assert_eq!(label, expected_label);
    let values = label_values.next().unwrap();
    values
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_group2(s: &str, expected_label: &str) -> u64 {
    let mut label_values = s.split(": ");
    let label = label_values.next().unwrap();
    assert_eq!(label, expected_label);
    let values = label_values.next().unwrap();
    values.trim().replace(" ", "").parse().unwrap()
}

pub fn calculate_wins(time: u64, distance: u64) -> u64 {
    let mut wins = 0;
    for speed in 1..time {
        let remaining_time = time - speed;
        let distance_covered = speed * remaining_time;
        if distance_covered > distance {
            wins += 1;
        }
    }
    wins
}

pub fn sixth_task_1(f: impl BufRead) -> u64 {
    let mut lines = f.lines().map(|l| l.unwrap());
    let times = parse_group(&lines.next().unwrap(), "Time");
    let distances = parse_group(&lines.next().unwrap(), "Distance");
    assert_eq!(times.len(), distances.len());
    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(t, d)| calculate_wins(t, d))
        .reduce(|a, b| a * b)
        .unwrap()
}

pub fn sixth_task_2(f: impl BufRead) -> u64 {
    let mut lines = f.lines().map(|l| l.unwrap());
    let time = parse_group2(&lines.next().unwrap(), "Time");
    let distance = parse_group2(&lines.next().unwrap(), "Distance");
    calculate_wins(time, distance)
}

//     let mut lines = f.lines().map(|l| l.unwrap());
