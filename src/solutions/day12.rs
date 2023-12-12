// use colored::Colorize;

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
        let inner_accum = solve_for(&map, 0, &dmg, 0, 0);
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
    dmg_cnt: usize,
) -> usize {
    if map_i == map.len() {
        if dmg_i == dmg.len() {
            // println!("{:?}", map);
            // println!("YES");
            return 1;
        } else {
            // println!("{:?}", map);
            // println!("FINISHED STRING SEARCH BUT DAMAGE NOT CONSUMED");
            return 0;
        }
    }
    if dmg_i > dmg.len() {
        return 0;
    }
    if dmg_i < dmg.len() && dmg_cnt > dmg[dmg_i] {
        return 0;
    }

    match map[map_i] {
        DAMAGE => {
            // print!("{}", DAMAGE);
            solve_for(map, map_i + 1, dmg, dmg_i, dmg_cnt + 1)
        }
        OPERATIONAL => {
            if dmg_cnt > 0 {
                if dmg_i >= dmg.len() {
                    // println!("{:?}", map);
                    // println!("TOO MANY DAMAGES");
                    0
                } else if dmg[dmg_i] != dmg_cnt {
                    // println!("{:?}", map);
                    // println!("DAMAGE STRING TOO LONG");
                    0
                } else {
                    // print!("{}", OPERATIONAL);
                    solve_for(map, map_i + 1, dmg, dmg_i + 1, 0)
                }
            } else {
                // print!("{}", OPERATIONAL);
                solve_for(map, map_i + 1, dmg, dmg_i, dmg_cnt)
            }
        }
        UNKNOWN => {
            // println!("{}", UNKNOWN);
            let try_op = if dmg_cnt > 0 {
                if dmg_i >= dmg.len() {
                    // print!("-");
                    0
                } else if dmg[dmg_i] != dmg_cnt {
                    // print!("_");
                    0
                } else {
                    solve_for(map, map_i + 1, dmg, dmg_i + 1, 0)
                }
            } else {
                solve_for(map, map_i + 1, dmg, dmg_i, dmg_cnt)
            };
            let try_dmg = solve_for(map, map_i + 1, dmg, dmg_i, dmg_cnt + 1);
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
        let inner_accum = solve_for(&map, 0, &dmg, 0, 0);
        println!("RESULT:");
        println!("{} {:?} = {}\n", spring_maps[i], dmg, inner_accum);
        accum += inner_accum;
    }

    final_answer(accum, submit, DAY, 2).await;
}
