use std::collections::HashSet;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 17;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    lines
}

fn lines_to_u8s(lines: Vec<String>) -> Vec<Vec<u8>> {
    let mut output = vec![];

    for line in lines {
        output.push(line.chars().map(|item| item as u8 - 0x30).collect());
    }

    output
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
    None,
}
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "N"),
            Direction::East => write!(f, "E"),
            Direction::South => write!(f, "S"),
            Direction::West => write!(f, "W"),
            Direction::None => write!(f, "_"),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Traveler {
    x: i64,
    y: i64,
    dir: Direction,
    num_steps_in_same_dir: usize,
    heat_lost: usize,
}
impl Traveler {
    fn new(x: i64, y: i64, dir: Direction) -> Self {
        Self {
            x,
            y,
            dir,
            num_steps_in_same_dir: 0,
            heat_lost: 0,
        }
    }
    fn can_travel(
        &self,
        dir: Direction,
        x_bounds: (usize, usize),
        y_bounds: (usize, usize),
    ) -> bool {
        if self.num_steps_in_same_dir > 3 && self.dir == dir {
            // can't take more than 3 steps in the same direction
            return false;
        }
        // prevent 180 degree turns
        match dir {
            Direction::North => {
                if dir == Direction::South {
                    return false;
                }
            }
            Direction::East => {
                if dir == Direction::West {
                    return false;
                }
            }
            Direction::South => {
                if dir == Direction::North {
                    return false;
                }
            }
            Direction::West => {
                if dir == Direction::East {
                    return false;
                }
            }
            Direction::None => {}
        }
        // bounds checks
        match dir {
            Direction::North => {
                return self.y > y_bounds.0 as i64;
            }
            Direction::East => {
                return self.x < x_bounds.1 as i64;
            }
            Direction::South => {
                return self.y < y_bounds.1 as i64;
            }
            Direction::West => {
                return self.x > x_bounds.0 as i64;
            }
            Direction::None => {
                return true;
            }
        }
        // if self.x > x_bounds.0 as i64 && self.x < x_bounds.1 as i64 {
        //     if self.y > y_bounds.0 as i64 && self.y < y_bounds.1 as i64 {
        //         return true;
        //     }
        // }

        // return false;
        // self.num_steps_in_same_dir > 2
        //     && self.dir == dir
        //     && self.x >= x_bounds.0 as i64
        //     && self.x <= x_bounds.1 as i64
        //     && self.y >= y_bounds.0 as i64
        //     && self.y <= y_bounds.1 as i64
    }
    fn travel(&mut self, dir: Direction) {
        if self.dir == dir {
            self.num_steps_in_same_dir += 1;
        } else {
            self.num_steps_in_same_dir = 0;
        }
        self.dir = dir;
        // print!("{}", self.dir);
        match self.dir {
            Direction::North => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
            Direction::None => {}
        }
    }
}

pub async fn d17s1(submit: bool, example: bool) {
    let lines = input(example).await;
    let heat_hits = lines_to_u8s(lines);
    let x_bounds: (usize, usize) = (0, heat_hits[0].len() - 1);
    let y_bounds: (usize, usize) = (0, heat_hits.len() - 1);
    let starting_traveler = Traveler::new(0, 0, Direction::None);
    let mut considering_paths = vec![starting_traveler];
    let mut best_heat_loss = vec![vec![usize::MAX; heat_hits[0].len()]; heat_hits.len()];

    let mut seen_travelers: HashSet<Traveler> = HashSet::new();

    while !considering_paths.is_empty() {
        if best_heat_loss[y_bounds.1 as usize][x_bounds.1 as usize] < usize::MAX {
            break;
        }
        let mut traveler = pop_best_heat(&mut considering_paths);
        traveler.heat_lost += heat_hits[traveler.y as usize][traveler.x as usize] as usize;
        println!("{}", traveler.heat_lost);
        // if traveler.y > y_bounds.1 as i64 || traveler.x > x_bounds.1 as i64 {
        //     continue;
        // }
        if traveler.heat_lost >= best_heat_loss[traveler.y as usize][traveler.x as usize] {
            // continue;
        } else {
            best_heat_loss[traveler.y as usize][traveler.x as usize] = traveler.heat_lost;
        }
        if traveler.can_travel(Direction::North, x_bounds, y_bounds) {
            let mut new_traveler = traveler.clone();
            new_traveler.travel(Direction::North);
            if !seen_travelers.contains(&new_traveler) {
                considering_paths.push(new_traveler.clone());
                seen_travelers.insert(new_traveler);
            }
            // println!("N");
        }
        if traveler.can_travel(Direction::East, x_bounds, y_bounds) {
            let mut new_traveler = traveler.clone();
            new_traveler.travel(Direction::East);
            if !seen_travelers.contains(&new_traveler) {
                considering_paths.push(new_traveler.clone());
                seen_travelers.insert(new_traveler);
            }
            // println!("E");
        }
        if traveler.can_travel(Direction::South, x_bounds, y_bounds) {
            let mut new_traveler = traveler.clone();
            new_traveler.travel(Direction::South);
            if !seen_travelers.contains(&new_traveler) {
                considering_paths.push(new_traveler.clone());
                seen_travelers.insert(new_traveler);
            }
            // println!("S");
        }
        if traveler.can_travel(Direction::West, x_bounds, y_bounds) {
            let mut new_traveler = traveler.clone();
            new_traveler.travel(Direction::West);
            if !seen_travelers.contains(&new_traveler) {
                considering_paths.push(new_traveler.clone());
                seen_travelers.insert(new_traveler);
            }
            // println!("W");
        }
    }

    final_answer(
        best_heat_loss[y_bounds.1 as usize][x_bounds.1 as usize],
        submit,
        DAY,
        1,
    )
    .await;
}

fn pop_best_heat(considering_paths: &mut Vec<Traveler>) -> Traveler {
    let mut best_heat = usize::MAX;
    let mut best_heat_index: usize = 0;
    for i in 0..considering_paths.len() {
        if considering_paths[i].heat_lost < best_heat {
            best_heat = considering_paths[i].heat_lost;
            best_heat_index = i;
        }
    }
    let trav = considering_paths[best_heat_index].clone();
    considering_paths.remove(best_heat_index);

    trav
}

pub async fn d17s2(submit: bool, example: bool) {
    let input = input(example).await;
    final_answer(input[0].to_owned(), submit, DAY, 2).await;
}
