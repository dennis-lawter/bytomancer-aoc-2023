use super::final_answer;
use super::input_raw;

const DAY: u8 = 11;

async fn input(example: bool) -> Vec<Vec<char>> {
    let raw = input_raw(DAY, example).await;
    let lines: Vec<String> = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    lines.iter().map(|item| item.chars().collect()).collect()
}

const EMPTY: char = '.';
const GALAXY: char = '#';

fn expand_input_rows(input: &mut Vec<Vec<char>>) {
    let mut output = vec![];
    for y in 0..input.len() {
        let mut has_seen_galaxy = false;
        for x in 0..input[0].len() {
            if input[y][x] == GALAXY {
                has_seen_galaxy = true;
                break;
            }
        }
        output.push(input[y].clone());
        if !has_seen_galaxy {
            output.push(input[y].clone());
        }
    }
    *input = output.clone();
}

fn flip_input(input: &mut Vec<Vec<char>>) {
    let mut output: Vec<Vec<char>> = vec![];

    for x in 0..input[0].len() {
        let new_row = vec![EMPTY; input.len()];
        output.push(new_row);
        for y in 0..input.len() {
            output[x][y] = input[y][x];
        }
    }

    *input = output.clone();
}

fn expand_input_cols(input: &mut Vec<Vec<char>>) {
    flip_input(input);
    expand_input_rows(input);
    flip_input(input);
}

fn expand_input(input: &mut Vec<Vec<char>>) {
    debug_input(&input);
    expand_input_rows(input);
    // debug_input(&input);
    expand_input_cols(input);
    debug_input(&input);
}

#[allow(dead_code)]
fn debug_input(input: &Vec<Vec<char>>) {
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            print!("{}", input[y][x]);
        }
        println!();
    }
    println!();
}

struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn manhattan_distance(&self, other: &Point) -> usize {
        self.y_distance(other) + self.x_distance(other)
    }
    fn y_distance(&self, other: &Point) -> usize {
        if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        }
    }
    fn x_distance(&self, other: &Point) -> usize {
        if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        }
    }
}

fn find_galaxies(input: &Vec<Vec<char>>) -> Vec<Point> {
    let mut output = vec![];
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == GALAXY {
                output.push(Point { x, y });
            }
        }
    }

    output
}

pub async fn d11s1(submit: bool, example: bool) {
    let mut input = input(example).await;
    expand_input(&mut input);
    let galaxies = find_galaxies(&input);
    let mut accum = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            accum += galaxies[i].manhattan_distance(&galaxies[j]);
        }
    }
    final_answer(accum, submit, DAY, 1).await;
}

// const EXPANSION_MULTIPLIER: usize = 10;
const EXPANSION_MULTIPLIER: usize = 1_000_000;

pub async fn d11s2(submit: bool, example: bool) {
    let mut input = input(example).await;
    let original_input = input.clone();
    expand_input(&mut input);
    let original_galaxies = find_galaxies(&original_input);
    let galaxies = find_galaxies(&input);
    let mut accum = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let original_distance = original_galaxies[i].manhattan_distance(&original_galaxies[j]);
            let expanded_distance = galaxies[i].manhattan_distance(&galaxies[j]);
            let expansion_difference = expanded_distance - original_distance;
            accum += original_distance + (expansion_difference * (EXPANSION_MULTIPLIER - 1));
        }
    }
    final_answer(accum, submit, DAY, 2).await;
}
