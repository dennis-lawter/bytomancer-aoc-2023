use std::collections::HashMap;
use std::collections::HashSet;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 14;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    lines
}

fn input_to_chars(input: Vec<String>) -> Vec<Vec<char>> {
    input.iter().map(|item| item.chars().collect()).collect()
}

const ROLL: char = 'O';
// const WALL: char = '#';
const EMPTY: char = '.';

pub async fn d14s1(submit: bool, example: bool) {
    let input = input(example).await;
    let mut chars = input_to_chars(input);
    roll_north(&mut chars);
    debug_char_map(&chars);
    let load = calc_north_load(&chars);
    final_answer(load, submit, DAY, 1).await;
}

fn are_equal(left: &Vec<Vec<char>>, right: &Vec<Vec<char>>) -> bool {
    for y in 0..left.len() {
        for x in 0..left[0].len() {
            if left[y][x] != right[y][x] {
                return false;
            }
        }
    }

    true
}

fn calc_north_load(chars: &Vec<Vec<char>>) -> usize {
    let mut output = 0;

    let mut row_val = chars.len();

    for y in 0..chars.len() {
        for x in 0..chars[0].len() {
            if chars[y][x] == ROLL {
                output += row_val;
            }
        }
        row_val -= 1;
    }

    output
}

fn debug_char_map(map: &Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!("{}", map[y][x]);
        }
        println!();
    }
    println!();
}

fn roll_north(chars: &mut Vec<Vec<char>>) {
    let mut rolled_rocks = 1;
    while rolled_rocks > 0 {
        rolled_rocks = 0;
        for y in 1..chars.len() {
            // can skip the upper row
            for x in 0..chars.len() {
                match chars[y][x] {
                    ROLL => {
                        if chars[y - 1][x] == EMPTY {
                            chars[y - 1][x] = ROLL;
                            chars[y][x] = EMPTY;
                            rolled_rocks += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn roll_south(chars: &mut Vec<Vec<char>>) {
    let mut rolled_rocks = 1;
    while rolled_rocks > 0 {
        rolled_rocks = 0;
        for y in 0..chars.len() - 1 {
            // can skip the upper row
            for x in 0..chars.len() {
                match chars[y][x] {
                    ROLL => {
                        if chars[y + 1][x] == EMPTY {
                            chars[y + 1][x] = ROLL;
                            chars[y][x] = EMPTY;
                            rolled_rocks += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn roll_east(chars: &mut Vec<Vec<char>>) {
    let mut rolled_rocks = 1;
    while rolled_rocks > 0 {
        rolled_rocks = 0;
        for y in 0..chars.len() {
            // can skip the upper row
            for x in 0..chars.len() - 1 {
                match chars[y][x] {
                    ROLL => {
                        if chars[y][x + 1] == EMPTY {
                            chars[y][x + 1] = ROLL;
                            chars[y][x] = EMPTY;
                            rolled_rocks += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn roll_west(chars: &mut Vec<Vec<char>>) {
    let mut rolled_rocks = 1;
    while rolled_rocks > 0 {
        rolled_rocks = 0;
        for y in 0..chars.len() {
            // can skip the upper row
            for x in 1..chars.len() {
                match chars[y][x] {
                    ROLL => {
                        if chars[y][x - 1] == EMPTY {
                            chars[y][x - 1] = ROLL;
                            chars[y][x] = EMPTY;
                            rolled_rocks += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn roll_spin_cycle(chars: &mut Vec<Vec<char>>) {
    roll_north(chars);
    roll_west(chars);
    roll_south(chars);
    roll_east(chars);
}

fn deep_clone(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new = Vec::with_capacity(map.len());
    for y in 0..map.len() {
        let row = map[y].clone();
        new.push(row);
    }

    new
}

fn hash_string(map: &Vec<Vec<char>>) -> String {
    let mut flat_vec: Vec<char> = Vec::with_capacity(map.len() * map[0].len());
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            flat_vec.push(map[y][x]);
        }
    }

    flat_vec.iter().collect()
}

const SPINS: usize = 1000000000;

pub async fn d14s2(submit: bool, example: bool) {
    let input = input(example).await;
    let mut chars = input_to_chars(input);

    let mut hash_set: HashMap<String, usize> = HashMap::new();
    hash_set.insert(hash_string(&chars), 0);
    // let mut previous_chars = deep_clone(&chars);
    debug_char_map(&chars);
    roll_spin_cycle(&mut chars);
    debug_char_map(&chars);
    let mut spins = 1;
    let mut traveling_hash = hash_string(&chars);
    while !hash_set.contains_key(&traveling_hash) {
        // previous_chars = deep_clone(&chars);
        hash_set.insert(traveling_hash.clone(), spins);
        roll_spin_cycle(&mut chars);
        debug_char_map(&chars);
        traveling_hash = hash_string(&chars);
        spins += 1;
    }
    let first_spin_matched = hash_set.get(&traveling_hash).unwrap();
    println!("Spins {} vs {} match", first_spin_matched, spins);
    let period = spins - first_spin_matched;
    // let period_advancement = spins;
    while spins + period < SPINS {
        spins += period;
    }
    while spins < SPINS {
        roll_spin_cycle(&mut chars);
        spins += 1;
    }

    // for _i in 0..SPINS {
    //     roll_spin_cycle(&mut chars);
    // }

    let load = calc_north_load(&chars);
    final_answer(load, submit, DAY, 2).await;
}
