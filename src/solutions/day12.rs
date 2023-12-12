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
    let cache_key = (map_i, dmg_i);
    match cache.get(&cache_key) {
        Some(cache_hit) => return *cache_hit,
        None => {}
    }
    let mut map_i = map_i;
    if map_i >= map.len() {
        if dmg_i == dmg.len() {
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
                }
            }
            if map_i + damages_to_consume < map.len() {
                if map[map_i + damages_to_consume] == DAMAGE {
                    cache.insert(cache_key, 0);
                    return 0;
                }
            }
            let res = solve_for(&map, map_i + damages_to_consume + 1, dmg, dmg_i + 1, cache);

            cache.insert(cache_key, res);

            res
        }
        UNKNOWN => {
            let mut try_dmg = 1;

            for i in map_i..map_i + damages_to_consume {
                if i >= map.len() {
                    try_dmg = 0;
                } else if map[i] == OPERATIONAL {
                    try_dmg = 0;
                }
            }
            if map_i + damages_to_consume < map.len() && map[map_i + damages_to_consume] == DAMAGE {
                try_dmg = 0;
            } else if try_dmg == 1 {
                try_dmg = solve_for(map, map_i + damages_to_consume + 1, dmg, dmg_i + 1, cache);
            }

            let try_op = solve_for(map, map_i + 1, dmg, dmg_i, cache);

            cache.insert(cache_key, try_op + try_dmg);
            try_op + try_dmg
        }
        _ => {
            panic!("Invalid char: {}", map[map_i])
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
