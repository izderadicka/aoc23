#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::day18::eighteens_task_1 as the_task;

mod day18;
fn main() {
    let file_name = std::env::args().nth(1).expect("Missing file name");
    let f = BufReader::new(File::open(file_name).expect("Problem opening file"));
    let res = the_task(f);
    println!("Result: {}", res);
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Game {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

impl Game {
    pub fn is_possible_result(&self, result: &Game) -> bool {
        self.red >= result.red && self.blue >= result.blue && self.green >= result.green
    }

    pub fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }

    pub fn max(&self, other: &Game) -> Game {
        Game {
            red: self.red.max(other.red),
            blue: self.blue.max(other.blue),
            green: self.green.max(other.green),
        }
    }
}

fn second_task_1() {
    let base_game = Game {
        red: 12,
        blue: 14,
        green: 13,
    };
    let f = load_input("input-day2.txt");
    let mut sum = 0;
    for line in f.lines() {
        let line = line.expect("Problem reading line");
        let (_input, (id, games)) = parser::parse_game_line(&line).unwrap();
        let are_all_games_possible = games.iter().all(|game| base_game.is_possible_result(game));
        println!("{} is possible: {}", line, are_all_games_possible);
        if are_all_games_possible {
            sum += id;
        }
    }
    println!("Result {}", sum);
}

fn second_task_2() {
    let f = load_input("input-day2.txt");
    let mut sum = 0;
    for line in f.lines() {
        let line = line.expect("Problem reading line");
        let (_input, (_id, games)) = parser::parse_game_line(&line).unwrap();
        let max_game = games
            .iter()
            .fold(Game::default(), |acc, game| acc.max(game));
        println!("{} max game: {:?}", line, max_game);

        sum += max_game.power();
    }
    println!("Result {}", sum);
}

fn load_input(file_name: impl AsRef<Path>) -> BufReader<File> {
    let f = BufReader::new(File::open(file_name).expect("Problem opening file"));
    return f;
}

fn first_task() {
    let f = BufReader::new(File::open("input-day1.txt").expect("Problem opening file"));
    let mut sum: u32 = 0;
    const DIGITS: &[&str] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in f.lines() {
        let mut numbers: Vec<u32> = vec![];
        let line = line.expect("Problem reading line");
        let mut buf = String::with_capacity(line.len());
        for c in line.chars() {
            if c.is_ascii_digit() {
                numbers.push(c.to_digit(10).unwrap());
            } else {
                buf.push(c);
                for i in 0..DIGITS.len() {
                    let digit_name = DIGITS[i];
                    let buf_len = buf.len();
                    let dig_len = digit_name.len();
                    if buf_len >= dig_len && &buf[buf_len - dig_len..buf_len] == digit_name {
                        numbers.push(i as u32 + 1);
                        if numbers.is_empty() {
                            buf = String::with_capacity(line.len());
                        }
                    }
                }
            }
        }
        let two_digit_num = numbers.first().unwrap() * 10 + numbers.last().unwrap();
        println!("{} = {}", line, two_digit_num);
        sum += two_digit_num;
    }
    println!("{}", sum)
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, multispace1},
        combinator::opt,
        multi::separated_list1,
        sequence::{preceded, separated_pair},
        Err::Failure,
        IResult,
    };

    use crate::Game;

    #[derive(Debug, PartialEq, Eq)]
    struct ColorCount {
        color: String,
        count: u32,
    }

    pub fn parse_game_line(input: &str) -> IResult<&str, (u32, Vec<Game>)> {
        let (input, id) = preceded(tag("Game "), digit1)(input)?;
        let id: u32 = id.parse().unwrap();
        let (input, _) = tag(":")(input)?;
        let (input, games) = separated_list1(tag(";"), parse_game)(input)?;
        Ok((input, (id, games)))
    }

    fn parse_color_count(input: &str) -> IResult<&str, ColorCount> {
        let (input, (count, color)) = preceded(
            opt(multispace1),
            separated_pair(
                digit1,
                multispace1,
                alt((tag("red"), tag("blue"), tag("green"))),
            ),
        )(input)?;
        let count: u32 = count.parse().unwrap();
        Ok((
            input,
            ColorCount {
                color: color.into(),
                count,
            },
        ))
    }

    fn parse_game(input: &str) -> IResult<&str, Game> {
        let (input, color_counts) = separated_list1(tag(","), parse_color_count)(input)?;
        let mut game = Game::default();
        for color_count in color_counts {
            match color_count.color.as_str() {
                "red" => game.red = color_count.count,
                "blue" => game.blue = color_count.count,
                "green" => game.green = color_count.count,
                _ => {
                    return Err(Failure(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Tag,
                    )))
                }
            }
        }
        Ok((input, game))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_game_line() {
            assert_eq!(
                parse_game_line("Game 1: 5 red, 3 blue, 2 green; 10 blue, 3 red, 2 green"),
                Ok((
                    "",
                    (
                        1,
                        vec![
                            Game {
                                red: 5,
                                blue: 3,
                                green: 2,
                            },
                            Game {
                                red: 3,
                                blue: 10,
                                green: 2,
                            }
                        ]
                    )
                ))
            );
            assert_eq!(
                parse_game_line("Game 2: 3 green, 2 red, 1 blue; 10 blue, 3 red, 2 green"),
                Ok((
                    "",
                    (
                        2,
                        vec![
                            Game {
                                red: 2,
                                blue: 1,
                                green: 3,
                            },
                            Game {
                                red: 3,
                                blue: 10,
                                green: 2,
                            }
                        ]
                    )
                ))
            );
        }

        #[test]
        fn test_parse_game() {
            assert_eq!(
                parse_game("5 red, 3 blue, 2 green"),
                Ok((
                    "",
                    Game {
                        red: 5,
                        blue: 3,
                        green: 2,
                    }
                ))
            );
            assert_eq!(
                parse_game("10 blue, 3 red, 2 green"),
                Ok((
                    "",
                    Game {
                        red: 3,
                        blue: 10,
                        green: 2,
                    }
                ))
            );
            assert_eq!(
                parse_game("3 green, 2 red, 1 blue"),
                Ok((
                    "",
                    Game {
                        red: 2,
                        blue: 1,
                        green: 3,
                    }
                ))
            );
        }

        #[test]
        fn test_parse_color_count() {
            assert_eq!(
                parse_color_count("5 red"),
                Ok((
                    "",
                    ColorCount {
                        color: "red".to_string(),
                        count: 5,
                    }
                ))
            );
            assert_eq!(
                parse_color_count("10 blue"),
                Ok((
                    "",
                    ColorCount {
                        color: "blue".to_string(),
                        count: 10,
                    }
                ))
            );
            assert_eq!(
                parse_color_count("3 green"),
                Ok((
                    "",
                    ColorCount {
                        color: "green".to_string(),
                        count: 3,
                    }
                ))
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_possible_result() {
        let game1 = Game {
            red: 5,
            blue: 3,
            green: 2,
        };
        let game2 = Game {
            red: 3,
            blue: 2,
            green: 1,
        };
        assert!(game1.is_possible_result(&game2));
    }
}
