use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 2;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw.lines().map(|item| item.to_owned()).collect();

    lines
}

pub async fn d02s1(submit: bool, example: bool) {
    let input = input(example).await;
    const RED_MAX: u64 = 12;
    const GREEN_MAX: u64 = 13;
    const BLUE_MAX: u64 = 14;
    let regex = Regex::new(r#"Game (\d+): (.*)"#).unwrap();
    let red_regex = Regex::new(r#"(\d+) red"#).unwrap();
    let green_regex = Regex::new(r#"(\d+) green"#).unwrap();
    let blue_regex = Regex::new(r#"(\d+) blue"#).unwrap();
    let mut accum = 0;
    for in_line in input {
        if in_line == "" {
            continue;
        }
        let caps = regex
            .captures(in_line.as_str())
            .expect(format!("Failure to parse: {}", &in_line).as_str());
        let id = str::parse::<u64>(caps.get(1).unwrap().as_str()).unwrap();
        let game_rounds_str = caps.get(2).unwrap().as_str().to_owned();
        let game_rounds: Vec<String> = game_rounds_str
            .split(";")
            .map(|item| item.to_owned())
            .collect();
        let mut valid_game = true;
        for game_round in game_rounds {
            match red_regex.captures(&game_round) {
                Some(red_match) => {
                    let red_cnt = str::parse::<u64>(red_match.get(1).unwrap().as_str()).unwrap();
                    if red_cnt > RED_MAX {
                        valid_game = false;
                    }
                }
                None => {}
            }
            match green_regex.captures(&game_round) {
                Some(green_match) => {
                    let green_cnt =
                        str::parse::<u64>(green_match.get(1).unwrap().as_str()).unwrap();
                    if green_cnt > GREEN_MAX {
                        valid_game = false;
                    }
                }
                None => {}
            }
            match blue_regex.captures(&game_round) {
                Some(blue_match) => {
                    let blue_cnt = str::parse::<u64>(blue_match.get(1).unwrap().as_str()).unwrap();
                    if blue_cnt > BLUE_MAX {
                        valid_game = false;
                    }
                }
                None => {}
            }
        }

        if valid_game {
            accum += id;
        }
    }
    final_answer(accum, submit, DAY, 1).await;
}

pub async fn d02s2(submit: bool, example: bool) {
    let input = input(example).await;
    let regex = Regex::new(r#"Game (\d+): (.*)"#).unwrap();
    let red_regex = Regex::new(r#"(\d+) red"#).unwrap();
    let green_regex = Regex::new(r#"(\d+) green"#).unwrap();
    let blue_regex = Regex::new(r#"(\d+) blue"#).unwrap();
    let mut accum = 0;
    for in_line in input {
        if in_line == "" {
            continue;
        }
        let caps = regex
            .captures(in_line.as_str())
            .expect(format!("Failure to parse: {}", &in_line).as_str());
        let id = str::parse::<u64>(caps.get(1).unwrap().as_str()).unwrap();
        let game_rounds_str = caps.get(2).unwrap().as_str().to_owned();
        let game_rounds: Vec<String> = game_rounds_str
            .split(";")
            .map(|item| item.to_owned())
            .collect();
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;
        for game_round in game_rounds {
            match red_regex.captures(&game_round) {
                Some(red_match) => {
                    let red_cnt = str::parse::<u64>(red_match.get(1).unwrap().as_str()).unwrap();
                    if red_cnt > red_max {
                        red_max = red_cnt;
                    }
                }
                None => {}
            }
            match green_regex.captures(&game_round) {
                Some(green_match) => {
                    let green_cnt =
                        str::parse::<u64>(green_match.get(1).unwrap().as_str()).unwrap();
                    if green_cnt > green_max {
                        green_max = green_cnt;
                    }
                }
                None => {}
            }
            match blue_regex.captures(&game_round) {
                Some(blue_match) => {
                    let blue_cnt = str::parse::<u64>(blue_match.get(1).unwrap().as_str()).unwrap();
                    if blue_cnt > blue_max {
                        blue_max = blue_cnt;
                    }
                }
                None => {}
            }
        }
        let game_power = red_max * blue_max * green_max;
        println!("Game id {} power: {}", id, game_power);
        accum += game_power;
    }
    final_answer(accum, submit, DAY, 2).await;
}
