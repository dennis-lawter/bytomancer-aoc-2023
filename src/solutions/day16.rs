use std::collections::HashSet;
use std::fmt::Display;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 16;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    lines
}

fn lines_to_chars(lines: Vec<String>) -> Vec<Vec<char>> {
    let mut output = vec![];

    for line in lines {
        output.push(line.chars().collect());
    }

    output
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "N"),
            Direction::East => write!(f, "E"),
            Direction::South => write!(f, "S"),
            Direction::West => write!(f, "W"),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Beam {
    x: i64,
    y: i64,
    dir: Direction,
}
impl Beam {
    fn new(x: i64, y: i64, dir: Direction) -> Self {
        Self { x, y, dir }
    }
    fn travel(&mut self) {
        print!("{}", self.dir);
        match self.dir {
            Direction::North => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
        }
    }
    fn split(&self) -> (Self, Self) {
        match self.dir {
            Direction::North | Direction::South => {
                let a = Self {
                    x: self.x,
                    y: self.y,
                    dir: Direction::East,
                };
                let b = Self {
                    x: self.x,
                    y: self.y,
                    dir: Direction::West,
                };
                (a, b)
            }
            Direction::East | Direction::West => {
                let a = Self {
                    x: self.x,
                    y: self.y,
                    dir: Direction::North,
                };
                let b = Self {
                    x: self.x,
                    y: self.y,
                    dir: Direction::South,
                };
                (a, b)
            }
        }
    }
}

const SPLIT_VERT: char = '|';
const SPLIT_HOR: char = '-';
const MIR_N2E_S2W: char = '/';
const MIR_N2W_S2E: char = '\\';

pub async fn d16s1(submit: bool, example: bool) {
    let input = input(example).await;
    let chars = lines_to_chars(input);

    let first_beam = Beam::new(-1, 0, Direction::East);
    let score = score_map(&chars, first_beam);

    final_answer(score, submit, DAY, 1).await;
}

pub async fn d16s2(submit: bool, example: bool) {
    let input = input(example).await;
    let chars = lines_to_chars(input);

    let mut first_beams: Vec<Beam> = vec![];

    let max_x = chars[0].len() as i64;
    let max_y = chars.len() as i64;
    for x in 0..max_x {
        first_beams.push(Beam::new(x, -1, Direction::South));
        first_beams.push(Beam::new(x, max_y, Direction::North));
    }
    for y in 0..max_y {
        first_beams.push(Beam::new(-1, y, Direction::East));
        first_beams.push(Beam::new(max_x, y, Direction::West));
    }

    let mut scores: Vec<usize> = vec![];
    for first_beam in first_beams {
        let score = score_map(&chars, first_beam);
        scores.push(score);
    }

    let greatest_score = scores.iter().max().unwrap().clone();

    final_answer(greatest_score, submit, DAY, 2).await;
}

fn score_map(chars: &Vec<Vec<char>>, first_beam: Beam) -> usize {
    let mut beams: Vec<Beam> = vec![first_beam];

    let max_x = chars[0].len() as i64;
    let max_y = chars.len() as i64;

    let mut locations_visited: HashSet<(i64, i64)> = HashSet::new();
    let mut beams_visited: HashSet<Beam> = HashSet::new();

    while !beams.is_empty() {
        let i = 0;
        // if beams[i].x < max_x && beams[i].y < max_y && beams[i].x >= 0 && beams[i].y >= 0 {
        //     if chars[beams[i].y as usize][beams[i].x as usize] == '.' {
        //         match beams[i].dir {
        //             Direction::North => chars[beams[i].y as usize][beams[i].x as usize] = '^',
        //             Direction::East => chars[beams[i].y as usize][beams[i].x as usize] = '>',
        //             Direction::South => chars[beams[i].y as usize][beams[i].x as usize] = 'v',
        //             Direction::West => chars[beams[i].y as usize][beams[i].x as usize] = '<',
        //         }
        //     }
        // }
        beams[i].travel();
        let x = &beams[i].x;
        let y = &beams[i].y;
        if *x >= max_x || *y >= max_y || *x < 0 || *y < 0 {
            beams.remove(i);
            continue;
        }
        locations_visited.insert((*x, *y));
        if !beams_visited.insert(beams[i].clone()) {
            beams.remove(i);
            continue;
        }
        match chars[*y as usize][*x as usize] {
            SPLIT_VERT => match beams[i].dir {
                Direction::East | Direction::West => {
                    let (a, b) = beams[i].split();
                    beams.push(a);
                    beams.push(b);
                    beams.remove(i);
                }
                _ => {}
            },
            SPLIT_HOR => match beams[i].dir {
                Direction::North | Direction::South => {
                    let (a, b) = beams[i].split();
                    beams.push(a);
                    beams.push(b);
                    beams.remove(i);
                }
                _ => {}
            },
            MIR_N2E_S2W => match beams[i].dir {
                Direction::North => beams[i].dir = Direction::East,
                Direction::East => beams[i].dir = Direction::North,
                Direction::South => beams[i].dir = Direction::West,
                Direction::West => beams[i].dir = Direction::South,
            },
            MIR_N2W_S2E => match beams[i].dir {
                Direction::North => beams[i].dir = Direction::West,
                Direction::East => beams[i].dir = Direction::South,
                Direction::South => beams[i].dir = Direction::East,
                Direction::West => beams[i].dir = Direction::North,
            },
            _ => {}
        }

        // println!("\n");

        // debug_char_map(&chars);

        // println!("\n");
    }

    println!("\n");

    debug_char_map(&chars);

    println!("\n");

    debug_visited_map(&locations_visited, max_x, max_y);

    locations_visited.len()
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

fn debug_visited_map(locations_visited: &HashSet<(i64, i64)>, max_x: i64, max_y: i64) {
    for y in 0..max_y {
        for x in 0..max_x {
            if locations_visited.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
