use super::final_answer;
use super::input_raw;

const DAY: u8 = 3;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw.lines().map(|item| item.to_owned()).collect();

    lines
}

pub async fn d03s1(submit: bool, example: bool) {
    const IGNORED_CHARS: [char; 11] = ['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let input = input(example).await;
    let grid: Vec<Vec<char>> = input
        .iter()
        .filter(|item| item.len() > 0)
        .map(|item| item.chars().collect())
        .collect();
    let mut part_num_accum: i64 = 0;
    let mut num_accum: i64 = 0;
    let mut digits: i64 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y].len() == 0 {}
            // print!("{}", grid[y][x]);
            if grid[y][x].is_ascii_digit() {
                num_accum *= 10;
                num_accum += grid[y][x] as i64 - 0x30;
                digits += 1;
            }
            if (!grid[y][x].is_ascii_digit() && num_accum != 0) || x == (grid[y].len() - 1) {
                // we just finished a number
                let x0 = (x as i64) - digits - 1;
                let y0 = (y as i64) - 1;
                let x1 = x as i64;
                let y1 = (y as i64) + 1;

                let mut is_part_num = false;
                print!("({},{}) to ({},{})\tChar search:", x0, y0, x1, y1);
                for suby in y0..=y1 {
                    for subx in x0..=x1 {
                        if suby < 0 || suby >= grid.len() as i64 {
                            continue;
                        }
                        if subx < 0 || subx >= grid[suby as usize].len() as i64 {
                            continue;
                        }
                        print!("{}", &grid[suby as usize][subx as usize]);
                        if !IGNORED_CHARS.contains(&grid[suby as usize][subx as usize]) {
                            is_part_num = true;
                        }
                    }
                }
                println!();
                // score
                if is_part_num {
                    part_num_accum += num_accum;
                    println!("adding {}", num_accum);
                } else {
                    println!("skipping {}", num_accum);
                }
                // reset
                num_accum = 0;
                digits = 0;
            }
        }

        // println!();
    }
    final_answer(part_num_accum, submit, DAY, 1).await;
}

pub async fn d03s2(submit: bool, example: bool) {
    let input = input(example).await;
    let grid: Vec<Vec<char>> = input
        .iter()
        .filter(|item| item.len() > 0)
        .map(|item| item.chars().collect())
        .collect();
    let mut gear_ratio_accum: i64 = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '*' {
                let x0 = x as i64 - 1;
                let x1 = x as i64 + 1;
                let y0 = y as i64 - 1;
                let y1 = y as i64 + 1;

                let mut topleft = (x0, y0);
                let mut botright = (x1, y1);

                let try_pos = vec![
                    (x as i64 - 1, y as i64 - 1),
                    (x as i64, y as i64 - 1),
                    (x as i64 + 1, y as i64 - 1),
                    (x as i64 - 1, y as i64),
                    (x as i64 + 1, y as i64),
                    (x as i64 - 1, y as i64 + 1),
                    (x as i64, y as i64 + 1),
                    (x as i64 + 1, y as i64 + 1),
                ];
                let mut try_pos_rev = try_pos.clone();
                try_pos_rev.reverse();

                for trying in try_pos {
                    if trying.0 >= 0
                        && trying.1 >= 0
                        && trying.1 < grid.len() as i64
                        && trying.0 < grid[trying.1 as usize].len() as i64
                        && grid[trying.1 as usize][trying.0 as usize].is_ascii_digit()
                    {
                        topleft = trying;
                        break;
                    }
                }

                for trying in try_pos_rev {
                    if trying.0 >= 0
                        && trying.1 >= 0
                        && trying.1 < grid.len() as i64
                        && trying.0 < grid[trying.1 as usize].len() as i64
                        && grid[trying.1 as usize][trying.0 as usize].is_ascii_digit()
                    {
                        botright = trying;
                        break;
                    }
                }

                let num_one: i64 = find_number_at_loc(&grid, topleft);
                let num_one_first_digit_pos = find_first_digit_from_loc(&grid, topleft);
                let num_two: i64 = find_number_at_loc(&grid, botright);
                let num_two_first_digit_pos = find_first_digit_from_loc(&grid, botright);
                if num_one_first_digit_pos == num_two_first_digit_pos {
                    continue;
                }
                println!(
                    "Gear found w/ {:?} & {:?}, result of {} x {} = {}",
                    topleft,
                    botright,
                    num_one,
                    num_two,
                    num_one * num_two
                );
                gear_ratio_accum += num_one * num_two;
            }
        }
    }
    final_answer(gear_ratio_accum, submit, DAY, 2).await;
}

fn find_first_digit_from_loc(grid: &[Vec<char>], pos: (i64, i64)) -> (i64, i64) {
    let mut x = pos.0;
    let y = pos.1;
    while x >= 0 {
        if !grid[y as usize][x as usize].is_ascii_digit() {
            return (x, y);
        }
        x -= 1;
    }
    return (x, y);
}

fn find_number_at_loc(grid: &[Vec<char>], pos: (i64, i64)) -> i64 {
    let mut x = pos.0;
    let y = pos.1;
    let mut number = grid[y as usize][x as usize] as i64 - 0x30;
    // scroll left
    let mut scroll_adjust = 10;
    x -= 1;
    while x >= 0 && grid[y as usize][x as usize].is_ascii_digit() {
        let new_digit = ((grid[y as usize][x as usize] as i64) - 0x30) * scroll_adjust;
        number = number + new_digit;
        scroll_adjust *= 10;
        x -= 1;
    }
    // scroll right
    x = pos.0;
    x += 1;
    while x < grid[y as usize].len() as i64 && grid[y as usize][x as usize].is_ascii_digit() {
        let new_digit = (grid[y as usize][x as usize] as i64) - 0x30;
        number = number * 10 + new_digit;
        x += 1;
    }

    number
}
