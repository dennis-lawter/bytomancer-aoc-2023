use colored::Colorize;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 12;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    lines
}

const DAMAGE: char = '#';
const OPERATIONAL: char = '.';
const UNKNOWN: char = '?';
// const PLACEHOLDER: char = '_';

fn string_satisfies_damages(map: &String, damages: &Vec<usize>) -> bool {
    let lazy_fix = String::from(format!("{}{}", map.as_str(), OPERATIONAL));
    let mut dmg_i = 0;
    let mut dmg_count = 0;
    for c in lazy_fix.chars() {
        match c {
            DAMAGE => {
                // if dmg_count == 0 {
                //     // start a damage line
                // } else {
                //     // continue adding damage
                //     dmg_count += 1;
                // }
                dmg_count += 1;
            }
            OPERATIONAL => {
                if dmg_count > 0 {
                    // do verification
                    if dmg_i >= damages.len() {
                        // println!("{}", format!("TOO LONG  {} {:?}", map, damages).on_red());
                        return false;
                    }
                    if damages[dmg_i] != dmg_count {
                        // println!("{}", format!("NO        {} {:?}", map, damages).on_red());
                        return false;
                    }

                    // advance to next group
                    dmg_i += 1;
                    dmg_count = 0;
                }
            }
            _ => {}
        }
    }

    if dmg_i == damages.len() {
        // println!("YES       {} {:?}", map, damages);
        return true;
    } else {
        // println!("NOT ENOUGH{} {:?}", map, damages);
        return false;
    }
}

fn mutated_unknowns(map: &String) -> Vec<String> {
    // if !mutated_unknowns(map).contains(UNKNOWN) {
    //     return vec![map.clone()];
    // }

    if !map.contains(UNKNOWN) {
        return vec![map.clone()];
    }

    let mut output: Vec<String> = vec![];
    // let mut map_cpy = map.clone();

    // while map_cpy.contains(UNKNOWN) {
    //     let i = map_cpy.find(UNKNOWN).unwrap();
    //     map_cpy.replace_range(i..i + 1, &String::from(PLACEHOLDER));
    //     let mut cpy_dmg = map_cpy.clone();
    //     cpy_dmg.replace_range(i..i + 1, &String::from(DAMAGE));
    //     let cpy_op = map_cpy.clone();
    //     map_cpy.replace_range(i..i + 1, &String::from(OPERATIONAL));
    //     output.push(cpy_dmg);
    //     output.push(cpy_op);
    // }

    let i = map.find(UNKNOWN).unwrap();
    // map.replace_range(i..i + 1, &String::from(PLACEHOLDER));
    let mut cpy_dmg = map.clone();
    cpy_dmg.replace_range(i..i + 1, &String::from(DAMAGE));
    let mut cpy_op = map.clone();
    cpy_op.replace_range(i..i + 1, &String::from(OPERATIONAL));
    output.append(&mut mutated_unknowns(&cpy_dmg));
    output.append(&mut mutated_unknowns(&cpy_op));

    output
}

pub async fn d12s1(submit: bool, example: bool) {
    let input = input(example).await;
    let mut spring_maps: Vec<String> = Vec::with_capacity(input.len());
    let mut damaged_springs: Vec<Vec<usize>> = Vec::with_capacity(input.len());
    let number_of_rows = input.len();
    for row in input {
        let split: Vec<String> = row.split(" ").map(str::to_owned).collect();
        let spring_row = split[0].clone();
        spring_maps.push(spring_row);
        // println!("TEST:{}", split[1])
        let damage_nums: Vec<usize> = split[1]
            .split(",")
            .map(|item| str::parse::<usize>(item).unwrap())
            .collect();
        damaged_springs.push(damage_nums);
    }

    let mut accum = 0;

    for i in 0..number_of_rows {
        let map = spring_maps[i].clone();
        let dmg = damaged_springs[i].clone();
        let mutated_maps = mutated_unknowns(&map);
        let mut inner_accum = 0;
        for mut_map in mutated_maps {
            if string_satisfies_damages(&mut_map, &dmg) {
                inner_accum += 1;
            }
        }
        println!("RESULT:");
        println!("{} {:?} = {}\n", map, dmg, inner_accum);
        accum += inner_accum;
    }

    final_answer(accum, submit, DAY, 1).await;
}

pub async fn d12s2(submit: bool, example: bool) {
    let input = input(example).await;
    let mut spring_maps: Vec<String> = Vec::with_capacity(input.len());
    let mut damaged_springs: Vec<Vec<usize>> = Vec::with_capacity(input.len());
    let number_of_rows = input.len();
    for row in input {
        let split: Vec<String> = row.split(" ").map(str::to_owned).collect();
        let spring_row = format!(
            "{}?{}?{}?{}?{}",
            split[0].clone(),
            split[0].clone(),
            split[0].clone(),
            split[0].clone(),
            split[0].clone()
        );
        spring_maps.push(spring_row);
        // println!("TEST:{}", split[1])
        let new_split_1 = format!(
            "{},{},{},{},{}",
            split[1], split[1], split[1], split[1], split[1]
        );
        let damage_nums: Vec<usize> = new_split_1
            .split(",")
            .map(|item| str::parse::<usize>(item).unwrap())
            .collect();
        damaged_springs.push(damage_nums);
    }

    let mut accum = 0;

    for i in 0..number_of_rows {
        let map = spring_maps[i].clone();
        let dmg = damaged_springs[i].clone();
        let mutated_maps = mutated_unknowns(&map);
        let mut inner_accum = 0;
        for mut_map in mutated_maps {
            if string_satisfies_damages(&mut_map, &dmg) {
                inner_accum += 1;
            }
        }
        println!("RESULT:");
        println!("{} {:?} = {}\n", map, dmg, inner_accum);
        accum += inner_accum;
    }

    final_answer(accum, submit, DAY, 2).await;
}
