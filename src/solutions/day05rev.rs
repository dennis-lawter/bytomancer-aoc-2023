use std::ops::Range;

use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 5;

#[derive(Debug)]
struct AlmanacMap {
    src_label: String,
    dst_label: String,
    src: Vec<Range<u64>>,
    dst: Vec<Range<u64>>,
}
impl AlmanacMap {
    fn from_string(input: &String) -> Self {
        let regex = Regex::new(r#"(.+)-to-(.+) map:"#).unwrap();
        let lines: Vec<&str> = input.lines().collect();
        let captures = regex.captures(lines[0]).unwrap();
        let src_label = captures.get(1).unwrap().as_str().to_owned();
        let dst_label = captures.get(2).unwrap().as_str().to_owned();
        let map_lines: Vec<MapLine> = lines
            .iter()
            .skip(1)
            .map(|item| MapLine::from_string(&item.to_owned().to_owned()))
            .collect();
        let mut src = Vec::with_capacity(map_lines.len());
        let mut dst = Vec::with_capacity(map_lines.len());
        for map_line in map_lines {
            src.push(map_line.src..map_line.src + map_line.len);
            dst.push(map_line.dst..map_line.dst + map_line.len);
        }
        Self {
            src_label,
            dst_label,
            src,
            dst,
        }
    }
}

#[derive(Debug)]
struct MapLine {
    src: u64,
    dst: u64,
    len: u64,
}
impl MapLine {
    fn from_string(input: &String) -> Self {
        let numbers: Vec<u64> = input
            .split(" ")
            .map(|item| {
                str::parse::<u64>(item).expect(format!("Failed to parse {}", item).as_str())
            })
            .collect();
        Self {
            src: numbers[1],
            dst: numbers[0],
            len: numbers[2],
        }
    }
}

async fn input(example: bool) -> (Vec<u64>, Vec<AlmanacMap>) {
    let raw = input_raw(DAY, example).await;
    let tables: Vec<String> = raw
        .split("\n\n")
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();
    let seeds_line = tables[0].to_owned();
    let seed_numbers_string = seeds_line.strip_prefix("seeds: ").unwrap();
    let seeds: Vec<u64> = seed_numbers_string
        .split(" ")
        .map(|item| str::parse::<u64>(item).expect(format!("Failed to parse {}", item).as_str()))
        .collect();
    let almanac_maps = tables
        .iter()
        .skip(1)
        .map(|item| AlmanacMap::from_string(item))
        .collect();

    (seeds, almanac_maps)
}

pub async fn d05s1(submit: bool, example: bool) {
    let (seeds, maps) = input(example).await;
    println!("{:?}", seeds);
    println!("{:?}", maps);

    let mut locations: Vec<u64> = Vec::with_capacity(seeds.len());

    for seed in seeds {
        locations.push(p1_solve_loc(seed, &maps, 0));
    }

    println!("LOCS: {:?}", locations);

    let min_loc = locations.iter().min().unwrap().to_owned();

    final_answer(min_loc, submit, DAY, 1).await;
}

fn p1_solve_loc(src: u64, maps: &Vec<AlmanacMap>, depth: usize) -> u64 {
    if depth == maps.len() {
        println!("Final depth result: {}", src);
        return src;
    }
    let map = &maps[depth];
    for i in 0..map.src.len() {
        let src_range = &map.src[i];
        let dst_range = &map.dst[i];
        if src_range.contains(&src) {
            let diff = src - src_range.start;
            let dst = dst_range.start + diff;
            println!(
                "{} number {} corresponds to {} number {}.",
                map.src_label, src, map.dst_label, dst
            );
            return p1_solve_loc(dst, maps, depth + 1);
        }
    }

    println!(
        "{} number {} defaults to {} number {}.",
        map.src_label, src, map.dst_label, src
    );
    p1_solve_loc(src, maps, depth + 1)
}

pub async fn d05s2(submit: bool, example: bool) {
    let (seed_data, maps) = input(example).await;

    let mut seed_ranges: Vec<Range<u64>> = Vec::new();

    let mut i = 0;
    while i < seed_data.len() {
        let start_seed = seed_data[i];
        let range_len = start_seed + seed_data[i + 1];
        seed_ranges.push(start_seed..range_len);

        i += 2;
    }

    let range_sort_closure = |left: &Range<u64>, right: &Range<u64>| -> std::cmp::Ordering {
        left.start.cmp(&right.start)
    };

    seed_ranges.sort_by(range_sort_closure);

    for map in maps {
        apply_map(&map, &mut seed_ranges);
    }

    println!("Seed ranges:");
    seed_ranges.iter().for_each(|item| println!("{:?}", item));

    final_answer("NaN", submit, DAY, 2).await;
}

fn apply_map(map: &AlmanacMap, seed_ranges: &[Range<u64>]) {
    for src_range in &map.src {
        for seed_range in seed_ranges {
            if src_range.start > seed_range.start {
                let (_left, _right) = range_split(seed_range, src_range.start);
            } else if src_range.start < seed_range.end {
                let (_left, _right) = range_split(seed_range, src_range.start);
            }
        }
    }
}

#[allow(dead_code)]
fn range_split(range: &Range<u64>, start_of_right_split: u64) -> (Range<u64>, Range<u64>) {
    let left = range.start..start_of_right_split;
    let right = start_of_right_split..range.end;
    (left, right)
}
