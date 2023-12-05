use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 4;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    lines
}

fn string_to_num_list(in_string: &String) -> Vec<u64> {
    in_string
        .split(" ")
        .filter(|item| item.len() > 0)
        .map(|item| str::parse::<u64>(item).expect(format!("Could not parse {}", item).as_str()))
        .collect()
}

pub async fn d04s1(submit: bool, example: bool) {
    let input = input(example).await;
    let mut accum = 0;
    let regex = Regex::new(r#"Card +(\d+): (.*)"#).unwrap();
    for in_line in input {
        let captures = regex.captures(&in_line);
        match captures {
            Some(captures_found) => {
                let _card = str::parse::<u64>(captures_found.get(1).unwrap().as_str()).unwrap();
                let numbers = captures_found.get(2).unwrap().as_str();
                let split_numbers: Vec<String> =
                    numbers.split("|").map(|item| item.to_owned()).collect();
                let winning: Vec<u64> = string_to_num_list(&split_numbers[0]);
                let scratches: Vec<u64> = string_to_num_list(&split_numbers[1]);
                let mut winning_count = 0;
                println!("{}", in_line);
                for scratch in scratches {
                    if winning.contains(&scratch) {
                        winning_count += 1;
                    }
                }
                if winning_count > 0 {
                    let points = 1 << (winning_count - 1);
                    accum += points;
                    println!("{} winners = {} points!", winning_count, points);
                } else {
                    println!("No winners");
                }
            }
            None => {}
        }
    }

    final_answer(accum, submit, DAY, 1).await;
}

pub async fn d04s2(submit: bool, example: bool) {
    let input = input(example).await;
    let mut accum = 0;
    let regex = Regex::new(r#"Card +(\d+): (.*)"#).unwrap();
    let mut card_clones = vec![1u64; input.len()];
    for in_line in input {
        let captures = regex.captures(&in_line);
        match captures {
            Some(captures_found) => {
                let card = str::parse::<u64>(captures_found.get(1).unwrap().as_str()).unwrap();
                let card_idx = card as usize - 1;
                let curr_clones = card_clones[card_idx];
                let numbers = captures_found.get(2).unwrap().as_str();
                let split_numbers: Vec<String> =
                    numbers.split("|").map(|item| item.to_owned()).collect();
                let winning: Vec<u64> = string_to_num_list(&split_numbers[0]);
                let scratches: Vec<u64> = string_to_num_list(&split_numbers[1]);
                let mut winning_count = 0;
                println!("{}", in_line);
                for scratch in scratches {
                    if winning.contains(&scratch) {
                        winning_count += 1;
                    }
                }
                for i in 1..=winning_count {
                    card_clones[card_idx + i] += curr_clones;
                }
                println!("Card {} cloned {} times", card, curr_clones);
            }
            None => {}
        }
    }

    for c in card_clones {
        accum += c;
    }

    final_answer(accum, submit, DAY, 2).await;
}
