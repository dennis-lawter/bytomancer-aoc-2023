use super::final_answer;
use super::input_raw;

const DAY: u8 = 1;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw.split("\n").map(|item| item.to_owned()).collect();

    lines
}

fn get_digits(haystack: &String) -> (u64, u64) {
    let mut num1 = 0;
    let mut num2 = 0;
    let hay_chars: Vec<char> = haystack.chars().collect();
    for hay_char in hay_chars {
        match str::parse::<u64>(format!("{}", hay_char).as_str()) {
            Ok(valid_digit) => {
                num1 = valid_digit;
                break;
            }
            Err(_) => {}
        }
    }
    let hay_chars: Vec<char> = haystack.chars().rev().collect();
    for hay_char in hay_chars {
        match str::parse::<u64>(format!("{}", hay_char).as_str()) {
            Ok(valid_digit) => {
                num2 = valid_digit;
                break;
            }
            Err(_) => {}
        }
    }
    (num1, num2)
}

pub async fn d01s1(submit: bool, example: bool) {
    let input = input(example).await;
    let nums: Vec<(u64, u64)> = input.iter().map(get_digits).collect();

    let mut accum: u64 = 0;

    for num_pair in nums {
        let number: u64 = num_pair.0 * 10 + num_pair.1;
        accum += number
    }

    final_answer(accum, submit, DAY, 1).await;
}

fn get_digits_with_spelled(haystack: &String) -> (u64, u64) {
    let digits: Vec<&str> = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let str_to_i: Vec<&str> = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let str_to_i_rev: Vec<&str> = vec![
        "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ];

    let mut num1: u64 = 0;
    let mut num2: u64 = 0;

    let mut hay = haystack.to_owned();
    'hayloop: while hay.len() > 0 {
        for i in 0..digits.len() {
            if hay.starts_with(digits[i]) {
                num1 = i as u64;
                break 'hayloop;
            }
        }
        for i in 0..str_to_i.len() {
            if hay.starts_with(str_to_i[i]) {
                num1 = i as u64;
                break 'hayloop;
            }
        }
        let slices = hay.split_at(1);
        hay = slices.1.to_owned();
    }

    let mut hay: String = haystack.chars().rev().collect();
    'hayloop: while hay.len() > 1 {
        for i in 0..digits.len() {
            if hay.starts_with(digits[i]) {
                num2 = i as u64;
                break 'hayloop;
            }
        }
        for i in 0..str_to_i_rev.len() {
            if hay.starts_with(str_to_i_rev[i]) {
                num2 = i as u64;
                break 'hayloop;
            }
        }
        let slices = hay.split_at(1);
        hay = slices.1.to_owned();
    }

    if num1 == 0 {
        num1 = num2;
    } else if num2 == 0 {
        num2 = num1;
    }

    (num1, num2)
}

pub async fn d01s2(submit: bool, example: bool) {
    let input = input(example).await;
    let nums: Vec<(u64, u64)> = input.iter().map(get_digits_with_spelled).collect();

    let mut accum: u64 = 0;

    for num_pair in nums {
        let number: u64 = num_pair.0 * 10 + num_pair.1;
        println!("{}", number);
        accum += number
    }
    final_answer(accum, submit, DAY, 2).await;
}
