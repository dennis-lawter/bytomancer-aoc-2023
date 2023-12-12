// use colored::Colorize;

use std::collections::HashMap;

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

// fn string_satisfies_damages(map: &String, damages: &Vec<usize>) -> bool {
//     let lazy_fix = String::from(format!("{}{}", map.as_str(), OPERATIONAL));
//     let mut dmg_i = 0;
//     let mut dmg_count = 0;
//     for c in lazy_fix.chars() {
//         match c {
//             DAMAGE => {
//                 // if dmg_count == 0 {
//                 //     // start a damage line
//                 // } else {
//                 //     // continue adding damage
//                 //     dmg_count += 1;
//                 // }
//                 dmg_count += 1;
//             }
//             OPERATIONAL => {
//                 if dmg_count > 0 {
//                     // do verification
//                     if dmg_i >= damages.len() {
//                         // println!("{}", format!("TOO LONG  {} {:?}", map, damages).on_red());
//                         return false;
//                     }
//                     if damages[dmg_i] != dmg_count {
//                         // println!("{}", format!("NO        {} {:?}", map, damages).on_red());
//                         return false;
//                     }

//                     // advance to next group
//                     dmg_i += 1;
//                     dmg_count = 0;
//                 }
//             }
//             _ => {}
//         }
//     }

//     if dmg_i == damages.len() {
//         // println!("YES       {} {:?}", map, damages);
//         return true;
//     } else {
//         // println!("NOT ENOUGH{} {:?}", map, damages);
//         return false;
//     }
// }

// fn mutated_unknowns(map: &String) -> Vec<String> {
//     // if !mutated_unknowns(map).contains(UNKNOWN) {
//     //     return vec![map.clone()];
//     // }

//     if !map.contains(UNKNOWN) {
//         return vec![map.clone()];
//     }

//     let mut output: Vec<String> = vec![];
//     // let mut map_cpy = map.clone();

//     // while map_cpy.contains(UNKNOWN) {
//     //     let i = map_cpy.find(UNKNOWN).unwrap();
//     //     map_cpy.replace_range(i..i + 1, &String::from(PLACEHOLDER));
//     //     let mut cpy_dmg = map_cpy.clone();
//     //     cpy_dmg.replace_range(i..i + 1, &String::from(DAMAGE));
//     //     let cpy_op = map_cpy.clone();
//     //     map_cpy.replace_range(i..i + 1, &String::from(OPERATIONAL));
//     //     output.push(cpy_dmg);
//     //     output.push(cpy_op);
//     // }

//     let i = map.find(UNKNOWN).unwrap();
//     // map.replace_range(i..i + 1, &String::from(PLACEHOLDER));
//     let mut cpy_dmg = map.clone();
//     cpy_dmg.replace_range(i..i + 1, &String::from(DAMAGE));
//     let mut cpy_op = map.clone();
//     cpy_op.replace_range(i..i + 1, &String::from(OPERATIONAL));
//     output.append(&mut mutated_unknowns(&cpy_dmg));
//     output.append(&mut mutated_unknowns(&cpy_op));

//     output
// }

pub async fn d12s1(submit: bool, example: bool) {
    let input = input(example).await;
    let mut spring_maps: Vec<String> = Vec::with_capacity(input.len());
    let mut damaged_springs: Vec<Vec<usize>> = Vec::with_capacity(input.len());
    let number_of_rows = input.len();
    for row in input {
        let split: Vec<String> = row.split(" ").map(str::to_owned).collect();
        let spring_row = format!("{}.", split[0].clone());
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
        let map: Vec<char> = spring_maps[i].chars().collect();
        let dmg = damaged_springs[i].clone();
        // let mut inner_accum = 0;
        // for mut_map in mutated_maps {
        //     if string_satisfies_damages(&mut_map, &dmg) {
        //         inner_accum += 1;
        //     }
        // }
        let mut cache = HashMap::new();
        let inner_accum = solve_for(&map, 0, &dmg, 0, &mut cache);
        println!("RESULT:");
        println!("{} {:?} = {}\n", spring_maps[i], dmg, inner_accum);
        accum += inner_accum;
    }

    final_answer(accum, submit, DAY, 1).await;
}

fn solve_for(
    map: &Vec<char>,
    map_i: usize,
    dmg: &Vec<usize>,
    dmg_i: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // println!("Solve for {:?} {} {:?} {}", map, map_i, dmg, dmg_i);
    // copy & mutate for debugging
    // let mut map = map.clone();
    let cache_key = (map_i, dmg_i);
    match cache.get(&cache_key) {
        Some(cache_hit) => return *cache_hit,
        None => {}
    }
    let mut map_i = map_i;
    if map_i >= map.len() {
        if dmg_i == dmg.len() {
            // println!("{:?}", map);
            // println!("Consumed string and damages");
            cache.insert(cache_key, 1);
            return 1;
        } else {
            cache.insert(cache_key, 0);
            return 0;
        }
    }
    if dmg_i > dmg.len() {
        cache.insert(cache_key, 0);
        return 0;
    }
    if dmg_i == dmg.len() {
        // has no more damage
        for i in map_i..map.len() {
            match map[i] {
                DAMAGE => {
                    cache.insert(cache_key, 0);
                    return 0;
                }
                _ => {}
            }
        }
        // println!("{:?}", map);
        // println!("Consumed damages, string has no more to give");

        cache.insert(cache_key, 1);
        return 1;
    }

    while map[map_i] == OPERATIONAL {
        map_i += 1;
        if map_i >= map.len() {
            cache.insert(cache_key, 0);
            return 0;
        }
    }

    let damages_to_consume = dmg[dmg_i];

    // println!("MATCH: {}", map[map_i]);

    match map[map_i] {
        DAMAGE => {
            for i in map_i..map_i + damages_to_consume {
                if i >= map.len() {
                    cache.insert(cache_key, 0);
                    return 0;
                }
                if map[i] == OPERATIONAL {
                    cache.insert(cache_key, 0);
                    return 0;
                } else {
                    // map[i] = DAMAGE;
                }
            }
            if map_i + damages_to_consume < map.len() {
                if map[map_i + damages_to_consume] == DAMAGE {
                    cache.insert(cache_key, 0);
                    return 0;
                } else {
                    // map[map_i + damages_to_consume] = OPERATIONAL;
                }
            }
            let res = solve_for(&map, map_i + damages_to_consume + 1, dmg, dmg_i + 1, cache);

            cache.insert(cache_key, res);

            res
        }
        UNKNOWN => {
            let mut try_dmg = 1;
            // let mut dmg_clone = map.clone();

            for i in map_i..map_i + damages_to_consume {
                if i >= map.len() {
                    // println!("AAA");
                    try_dmg = 0;
                } else if map[i] == OPERATIONAL {
                    // println!("BBB");
                    try_dmg = 0;
                }
                // dmg_clone[i] = DAMAGE;
            }
            if map_i + damages_to_consume < map.len() && map[map_i + damages_to_consume] == DAMAGE {
                try_dmg = 0;
            } else if try_dmg == 1 {
                // dmg_clone[map_i + damages_to_consume] = OPERATIONAL;
                try_dmg = solve_for(map, map_i + damages_to_consume + 1, dmg, dmg_i + 1, cache);
            }

            // let mut op_clone = map.clone();
            // op_clone[map_i] = OPERATIONAL;
            let try_op = solve_for(map, map_i + 1, dmg, dmg_i, cache);

            cache.insert(cache_key, try_op + try_dmg);
            try_op + try_dmg
            // let mut clone_op = map.clone();
            // clone_op[map_i] = OPERATIONAL;

            // let mut clone_dmg = map.clone();
            // clone_dmg[map_i] = DAMAGE;

            // solve_for(&clone_op, map_i, dmg, dmg_i, dmg_cnt)
            //     + solve_for(&clone_dmg, map_i, dmg, dmg_i, dmg_cnt)
        }
        _ => {
            panic!("WHAT IS THIS: {}", map[map_i])
        }
    }
}

pub async fn d12s2(submit: bool, example: bool) {
    let input = input(example).await;
    let mut spring_maps: Vec<String> = Vec::with_capacity(input.len());
    let mut damaged_springs: Vec<Vec<usize>> = Vec::with_capacity(input.len());
    let number_of_rows = input.len();
    for row in input {
        let split: Vec<String> = row.split(" ").map(str::to_owned).collect();
        let spring_row = format!(
            "{}?{}?{}?{}?{}.",
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
        let map: Vec<char> = spring_maps[i].chars().collect();
        let dmg = damaged_springs[i].clone();
        // let mut inner_accum = 0;
        // for mut_map in mutated_maps {
        //     if string_satisfies_damages(&mut_map, &dmg) {
        //         inner_accum += 1;
        //     }
        // }
        println!("SOLVING {} of {}:", i + 1, number_of_rows);
        let input_string: String = map.iter().collect();
        println!("INPUT: {}\n{:?}", input_string, dmg);
        let mut cache = HashMap::new();
        let inner_accum = solve_for(&map, 0, &dmg, 0, &mut cache);
        println!("RESULT: {}", inner_accum);
        println!();
        accum += inner_accum;
    }

    final_answer(accum, submit, DAY, 2).await;
}
