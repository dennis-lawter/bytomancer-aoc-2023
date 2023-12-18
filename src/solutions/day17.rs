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

#[derive(Clone, Eq, PartialEq)]
struct Traveler {
    x: i64,
    y: i64,
    dir: Direction,
    num_steps_in_same_dir: usize,
    heat_lost: usize,
    // min_step_limit: usize,
    max_step_limit: usize,
    // history: Vec<Direction>,
}
impl Traveler {
    fn new(x: i64, y: i64, dir: Direction, max_step_limit: usize) -> Self {
        Self {
            x,
            y,
            dir,
            num_steps_in_same_dir: 0,
            heat_lost: 0,
            // min_step_limit,
            max_step_limit,
            // history: vec![],
        }
    }
    fn can_travel(
        &self,
        dir: Direction,
        x_bounds: (usize, usize),
        y_bounds: (usize, usize),
    ) -> bool {
        if self.num_steps_in_same_dir >= self.max_step_limit && self.dir == dir {
            return false;
        }
        // else if self.num_steps_in_same_dir < self.min_step_limit && self.dir != dir {
        //     return false;
        // }
        // prevent 180 degree turns
        match self.dir {
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
    fn travel(&mut self, dir: Direction, heat_hits: &Vec<Vec<u8>>) -> bool {
        // self.history.push(dir.clone());
        if self.dir == dir {
            self.num_steps_in_same_dir += 1;
        } else {
            self.num_steps_in_same_dir = 1;
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
        if self.y >= heat_hits.len() as i64
            || self.x >= heat_hits[0].len() as i64
            || self.y < 0
            || self.x < 0
        {
            return false;
        }
        self.heat_lost += heat_hits[self.y as usize][self.x as usize] as usize;

        true
    }
}
impl std::hash::Hash for Traveler {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.dir.hash(state);
        self.num_steps_in_same_dir.hash(state);
        // self.heat_lost.hash(state);
        // self.max_step_limit.hash(state);
    }
}
impl std::fmt::Display for Traveler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} heat, ", self.heat_lost)?;
        // for h in self.history.iter() {
        //     write!(f, "{}", h)?;
        // }

        Ok(())
    }
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

// fn pop_best_heat_with_dist(
//     considering_paths: &mut Vec<Traveler>,
//     manhattan_dist_to_end: usize,
// ) -> Traveler {
//     let mut best_heat = usize::MAX;
//     let mut best_heat_index: usize = 0;
//     for i in 0..considering_paths.len() {
//         let my_dist = manhattan_dist_to_end
//             - considering_paths[i].y as usize
//             - considering_paths[i].x as usize;
//         if my_dist == 0 {
//             // // this guy is standing at the point
//             // // wait don't early return in case there's a tie
//             // let trav = considering_paths[i].clone();
//             // considering_paths.remove(i);

//             // return trav;
//         } else if my_dist < 4 {
//             continue;
//         }
//         if considering_paths[i].heat_lost + my_dist < best_heat {
//             best_heat = considering_paths[i].heat_lost + my_dist;
//             best_heat_index = i;
//         }
//     }
//     let trav = considering_paths[best_heat_index].clone();
//     considering_paths.remove(best_heat_index);

//     trav
// }

const IMPOSSIBLE_RETRACE: usize = 9;

pub async fn d17s1(submit: bool, example: bool) {
    let lines = input(example).await;
    let heat_hits = lines_to_u8s(lines);
    let x_bounds: (usize, usize) = (0, heat_hits[0].len() - 1);
    let y_bounds: (usize, usize) = (0, heat_hits.len() - 1);
    let starting_traveler = Traveler::new(0, 0, Direction::None, 3);
    let mut considering_paths = vec![starting_traveler];
    let mut best_heat_loss = vec![vec![usize::MAX; heat_hits[0].len()]; heat_hits.len()];

    let mut seen_travelers: HashSet<Traveler> = HashSet::new();

    // let mut debug_futility = 0;

    while !considering_paths.is_empty() {
        // if debug_futility > 10 {
        //     break;
        // } else {
        //     debug_futility += 1;
        // }
        if best_heat_loss[y_bounds.1 as usize][x_bounds.1 as usize] < usize::MAX {
            break;
        }
        let traveler = pop_best_heat(&mut considering_paths);
        // println!("{}", traveler);
        // if traveler.y > y_bounds.1 as i64 || traveler.x > x_bounds.1 as i64 {
        //     continue;
        // }

        if traveler.heat_lost < best_heat_loss[traveler.y as usize][traveler.x as usize] {
            best_heat_loss[traveler.y as usize][traveler.x as usize] = traveler.heat_lost;
        }
        if traveler.heat_lost
            > best_heat_loss[traveler.y as usize][traveler.x as usize] + IMPOSSIBLE_RETRACE
        {
            continue;
        }
        if traveler.can_travel(Direction::North, x_bounds, y_bounds) {
            let mut new_traveler = traveler.clone();
            if new_traveler.travel(Direction::North, &heat_hits) {
                // new_traveler.heat_lost +=
                //     heat_hits[new_traveler.y as usize][new_traveler.x as usize] as usize;
                if !seen_travelers.contains(&new_traveler) {
                    considering_paths.push(new_traveler.clone());
                    seen_travelers.insert(new_traveler);
                }
            }
            // println!("N");
        }
        if traveler.can_travel(Direction::East, x_bounds, y_bounds) {
            let mut new_traveler = traveler.clone();
            if new_traveler.travel(Direction::East, &heat_hits) {
                // new_traveler.heat_lost +=
                //     heat_hits[new_traveler.y as usize][new_traveler.x as usize] as usize;
                if !seen_travelers.contains(&new_traveler) {
                    considering_paths.push(new_traveler.clone());
                    seen_travelers.insert(new_traveler);
                }
            }
            // println!("E");
        }
        if traveler.can_travel(Direction::South, x_bounds, y_bounds) {
            let mut new_traveler = traveler.clone();
            if new_traveler.travel(Direction::South, &heat_hits) {
                // new_traveler.heat_lost +=
                //     heat_hits[new_traveler.y as usize][new_traveler.x as usize] as usize;
                if !seen_travelers.contains(&new_traveler) {
                    considering_paths.push(new_traveler.clone());
                    seen_travelers.insert(new_traveler);
                }
            }
            // println!("S");
        }
        if traveler.can_travel(Direction::West, x_bounds, y_bounds) {
            let mut new_traveler = traveler.clone();
            if new_traveler.travel(Direction::West, &heat_hits) {
                // new_traveler.heat_lost +=
                //     heat_hits[new_traveler.y as usize][new_traveler.x as usize] as usize;
                if !seen_travelers.contains(&new_traveler) {
                    considering_paths.push(new_traveler.clone());
                    seen_travelers.insert(new_traveler);
                }
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

#[derive(Default)]
struct TravelerStorage {
    storage: Vec<Vec<Traveler>>,
    seen_travelers: HashSet<Traveler>,
    start: usize,
}
impl TravelerStorage {
    fn insert(&mut self, trav: Traveler, upper_bounds: (usize, usize)) {
        if self.seen_travelers.contains(&trav) {
            return;
        }
        self.seen_travelers.insert(trav.clone());
        let distance_to_goal =
            (upper_bounds.0 - trav.x as usize) + (upper_bounds.1 - trav.y as usize);
        if distance_to_goal > 0 && distance_to_goal < 4 {
            return;
        }
        let i = trav.heat_lost + distance_to_goal;
        while self.storage.len() <= i {
            self.storage.push(vec![]);
        }
        self.storage[i].push(trav);
        if i < self.start {
            self.start = i;
        }
    }
    fn pop(&mut self) -> Traveler {
        for i in self.start..self.storage.len() {
            if self.storage[i].is_empty() {
                self.start += 1;
                // continue;
            } else {
                return self.storage[i].pop().unwrap();
            }
        }
        panic!("Uhhh");
    }
}

pub async fn d17s2(submit: bool, example: bool) {
    let lines = input(example).await;
    let heat_hits = lines_to_u8s(lines);
    let x_bounds: (usize, usize) = (0, heat_hits[0].len() - 1);
    let y_bounds: (usize, usize) = (0, heat_hits.len() - 1);
    let starting_traveler = Traveler::new(0, 0, Direction::None, 10);
    let mut best_heat_loss = vec![vec![usize::MAX; heat_hits[0].len()]; heat_hits.len()];

    let mut considering_paths = TravelerStorage::default();
    let upper_bounds = (x_bounds.1, y_bounds.1);
    considering_paths.insert(starting_traveler, upper_bounds);

    // let mut seen_travelers: HashSet<Traveler> = HashSet::new();

    // let mut debug_futility = 0;

    let mut debug_i = 0;

    // let mut new_considering_paths = considering_paths.clone();

    while best_heat_loss[y_bounds.1 as usize][x_bounds.1 as usize] == usize::MAX {
        // while !considering_paths.is_empty() {
        // if debug_futility > 10 {
        //     break;
        // } else {
        //     debug_futility += 1;
        // }
        let traveler = considering_paths.pop();
        // if seen_travelers.contains(&traveler) {
        //     continue;
        // } else {
        //     seen_travelers.insert(traveler.clone());
        // }
        if debug_i % 10_000 == 0 {
            println!("{}", traveler);
        }
        debug_i += 1;
        // println!("{}", traveler);
        if traveler.y < 0
            || traveler.x < 0
            || traveler.y > y_bounds.1 as i64
            || traveler.x > x_bounds.1 as i64
        {
            continue;
        }

        if traveler.heat_lost < best_heat_loss[traveler.y as usize][traveler.x as usize] {
            best_heat_loss[traveler.y as usize][traveler.x as usize] = traveler.heat_lost;
        } else {
            // continue; // Verified this is not correct
        }
        // if traveler.heat_lost
        //     > best_heat_loss[traveler.y as usize][traveler.x as usize] + IMPOSSIBLE_RETRACE * 4
        // {
        //     continue;
        // }
        match traveler.dir {
            Direction::North | Direction::South => {
                spawn_travelers(
                    &traveler,
                    Direction::East,
                    &mut considering_paths,
                    &heat_hits,
                );
                spawn_travelers(
                    &traveler,
                    Direction::West,
                    &mut considering_paths,
                    &heat_hits,
                );
            }
            Direction::East | Direction::West => {
                spawn_travelers(
                    &traveler,
                    Direction::North,
                    &mut considering_paths,
                    &heat_hits,
                );
                spawn_travelers(
                    &traveler,
                    Direction::South,
                    &mut considering_paths,
                    &heat_hits,
                );
            }
            Direction::None => {
                spawn_travelers(
                    &traveler,
                    Direction::East,
                    &mut considering_paths,
                    &heat_hits,
                );
                spawn_travelers(
                    &traveler,
                    Direction::South,
                    &mut considering_paths,
                    &heat_hits,
                );
            } // }
        }
        // considering_paths = new_considering_paths.clone();
        // new_considering_paths = vec![];
    }

    final_answer(
        best_heat_loss[y_bounds.1 as usize][x_bounds.1 as usize],
        submit,
        DAY,
        2,
    )
    .await;
}

fn spawn_travelers(
    traveler: &Traveler,
    dir: Direction,
    considering_paths: &mut TravelerStorage,
    heat_hits: &Vec<Vec<u8>>,
) {
    let mut new_traveler = traveler.clone();
    let mut success = true;
    let mut i = 0;
    let upper_bounds = (heat_hits[0].len() - 1, heat_hits.len() - 1);
    // march 4 times
    while success && i < 4 {
        success = new_traveler.travel(dir.clone(), &heat_hits);
        i += 1;
    }
    if success {
        considering_paths.insert(new_traveler.clone(), upper_bounds);
    }
    while success && i < 10 {
        success = new_traveler.travel(dir.clone(), &heat_hits);
        i += 1;

        if success {
            considering_paths.insert(new_traveler.clone(), upper_bounds);
        }
    }
}
