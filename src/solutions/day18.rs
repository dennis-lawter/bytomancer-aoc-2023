use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 18;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    lines
}

enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn from_str(input: &str) -> Self {
        match input {
            "U" => Self::North,
            "R" => Self::East,
            "D" => Self::South,
            "L" => Self::West,
            bad_input => panic!("Not a valid direction string: {}", bad_input),
        }
    }
}

struct DigPlan {
    dir: Direction,
    distance: u32,
    color: u32,
}
impl DigPlan {
    fn from_str(input: &str) -> Self {
        let regex = Regex::new(r#"^(.) (\d+) \(#(.+)\)$"#).unwrap();
        let captures = regex.captures(input).unwrap();
        let dir = Direction::from_str(captures.get(1).unwrap().as_str());
        let distance = str::parse::<u32>(captures.get(2).unwrap().as_str()).unwrap();
        let color = u32::from_str_radix(captures.get(3).unwrap().as_str(), 16).unwrap();

        Self {
            dir,
            distance,
            color,
        }
    }
}

pub async fn d18s1(submit: bool, example: bool) {
    let lines = input(example).await;
    let mut points: Vec<(i32, i32)> = Vec::with_capacity(lines.len());
    let mut last_point = (0, 0);
    let mut dig_plans: Vec<DigPlan> = Vec::with_capacity(lines.len());
    points.push(last_point.clone());
    for line in lines {
        let plan = DigPlan::from_str(line.as_str());
        match plan.dir {
            Direction::North => last_point.1 -= plan.distance as i32,
            Direction::East => last_point.0 += plan.distance as i32,
            Direction::South => last_point.1 += plan.distance as i32,
            Direction::West => last_point.0 -= plan.distance as i32,
        }
        dig_plans.push(plan);
        points.push(last_point);
    }
    let mut x_bounds = (0, 0);
    let mut y_bounds = (0, 0);
    for point in points.iter() {
        if point.0 < x_bounds.0 {
            x_bounds.0 = point.0;
        }
        if point.1 < y_bounds.0 {
            y_bounds.0 = point.1;
        }
        if point.0 > x_bounds.1 {
            x_bounds.1 = point.0;
        }
        if point.1 > y_bounds.1 {
            y_bounds.1 = point.1;
        }
    }
    let height = 2 * (y_bounds.1 - y_bounds.0) as usize;
    let width = 2 * (x_bounds.1 - x_bounds.0) as usize;
    println!(
        "DEBUG:\n\n{:?}\n\n{:?}\n\n{:?}\n\n{}x{}",
        points, x_bounds, y_bounds, width, height
    );
    let mut grid = vec![vec![false; width]; height];
    let mut cursor = (0 - x_bounds.0, 0 - y_bounds.0);
    for plan in dig_plans {
        for _i in 0..plan.distance {
            match plan.dir {
                Direction::North => cursor.1 -= 1,
                Direction::East => cursor.0 += 1,
                Direction::South => cursor.1 += 1,
                Direction::West => cursor.0 -= 1,
            }
            grid[(cursor.1 - y_bounds.0) as usize][(cursor.0 - x_bounds.0) as usize] = true;
        }
    }
    let mut flood_start = (0, 0);
    'find_flood_start: for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] {
                flood_start = (x, y);
                break 'find_flood_start;
            }
        }
    }
    // move the flood start 1 unit SE to hit a contained empty patch
    flood_start.0 += 1;
    flood_start.1 += 1;
    println!();
    debug_grid(&grid);
    flood_grid(&mut grid, flood_start);
    let answer = find_filled(&grid);
    println!();
    debug_grid(&grid);
    final_answer(answer, submit, DAY, 1).await;
}

fn debug_grid(grid: &Vec<Vec<bool>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn find_filled(grid: &Vec<Vec<bool>>) -> usize {
    let mut count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] {
                count += 1;
            }
        }
    }

    count
}

fn flood_grid(grid: &mut Vec<Vec<bool>>, flood_at: (usize, usize)) {
    let y = flood_at.1;
    let x = flood_at.0;
    grid[y][x] = true;
    if y > 0 {
        if grid[y - 1][x] == false {
            flood_grid(grid, (x, y - 1));
        }
    }
    if y < grid.len() - 1 {
        if grid[y + 1][x] == false {
            flood_grid(grid, (x, y + 1));
        }
    }
    if x > 0 {
        if grid[y][x - 1] == false {
            flood_grid(grid, (x - 1, y));
        }
    }
    if x < grid[0].len() - 1 {
        if grid[y][x + 1] == false {
            flood_grid(grid, (x + 1, y));
        }
    }

    // println!();
    // debug_grid(grid.as_ref());
    // println!();
}

pub async fn d18s2(submit: bool, example: bool) {
    let input = input(example).await;
    final_answer(input[0].to_owned(), submit, DAY, 2).await;
}
