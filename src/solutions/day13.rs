use super::final_answer;
use super::input_raw;

const DAY: u8 = 13;

async fn input(example: bool) -> Vec<Vec<Vec<char>>> {
    let raw = input_raw(DAY, example).await;
    let mut output = vec![];
    let groups = raw
        .split("\n\n")
        .map(str::to_owned)
        .collect::<Vec<String>>();
    for group in groups {
        let line = group.lines().map(|item| item.chars().collect()).collect();
        output.push(line);
    }

    output
}

fn get_rotations(map: &Vec<Vec<char>>) -> (Vec<String>, Vec<String>) {
    let mut normal = vec![];
    let mut rotated = vec![];
    for y in 0..map.len() {
        normal.push(map[y].iter().collect());
    }
    for x in 0..map[0].len() {
        let mut vert_chars: Vec<char> = vec![];
        for y in 0..map.len() {
            vert_chars.push(map[y][x]);
        }
        rotated.push(vert_chars.iter().collect());
    }
    (normal, rotated)
}

pub async fn d13s1(submit: bool, example: bool) {
    let input = input(example).await;
    let mut accum: usize = 0;
    for map in input {
        accum += score_map(&map)[0];
    }
    final_answer(accum, submit, DAY, 1).await;
}

fn score_map(map: &Vec<Vec<char>>) -> Vec<usize> {
    let (normal, rotated) = get_rotations(&map);
    let mut normal_reflections = get_tb_reflection(&normal);
    for i in 0..normal_reflections.len() {
        normal_reflections[i] *= 100;
    }
    let mut rotated_reflections = get_tb_reflection(&rotated);
    normal_reflections.append(&mut rotated_reflections);
    normal_reflections
}
fn score_map_avoiding(map: &Vec<Vec<char>>, avoid: usize) -> Option<usize> {
    let scores = score_map(map);
    for score in scores {
        if score != avoid {
            return Some(score);
        }
    }
    None
}

fn get_tb_reflection(map: &Vec<String>) -> Vec<usize> {
    let mut solutions = vec![];
    'row_loop: for y in 1..map.len() {
        let mut y0 = y - 1;
        let mut y1 = y;
        while y0 != usize::MAX && y1 < map.len() {
            if map[y0] != map[y1] {
                continue 'row_loop;
            }
            y0 -= 1;
            y1 += 1;
        }
        solutions.push(y);
    }
    solutions
}

enum MapTile {
    Ash = '.' as isize,
    Rock = '#' as isize,
}
impl TryFrom<char> for MapTile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(MapTile::Ash),
            '#' => Ok(MapTile::Rock),
            _ => Err(()),
        }
    }
}

fn generate_permutations(map: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let width = map[0].len();
    let height = map.len();
    let mut output = Vec::with_capacity(width * height);

    for i in 0..width * height {
        let mut cloned_map = deep_clone(&map);
        let y = i / width;
        let x = i % width;
        match TryInto::<MapTile>::try_into(map[y][x]) {
            Ok(valid_tile) => match valid_tile {
                MapTile::Ash => cloned_map[y][x] = MapTile::Rock as u8 as char,
                MapTile::Rock => cloned_map[y][x] = MapTile::Ash as u8 as char,
            },
            Err(_) => todo!(),
        }
        output.push(cloned_map);
    }

    output
}

fn deep_clone(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new = Vec::with_capacity(map.len());
    for y in 0..map.len() {
        let row = map[y].clone();
        new.push(row);
    }

    new
}

pub async fn d13s2(submit: bool, example: bool) {
    let input = input(example).await;
    let mut accum: usize = 0;
    for src_map in input {
        let original_score = score_map(&src_map)[0];
        let permutations = generate_permutations(&src_map);
        let new_score_result = find_perm_score(&permutations, original_score);
        match new_score_result {
            Some(new_score) => {
                accum += new_score;
            }
            None => {
                let (orig, rot) = get_rotations(&src_map);
                println!("ORIGINAL MAP:");
                debug_map(&orig);
                println!("\nROTATED MAP:");
                debug_map(&rot);
                println!("Original score: {}", original_score);
                panic!("REVIEW THE ABOVE MAP");
            }
        }
        // let mut perm_num = 0;

        // let (normal, rotated) = get_rotations(&map);
        // match get_tb_reflection(&normal) {
        //     Some(reflection) => {
        //         accum += reflection * 100;
        //         println!("Perm # {} scored {}", perm_num, reflection * 100);
        //         break 'seek_perm;
        //     }
        //     None => match get_tb_reflection(&rotated) {
        //         Some(reflection) => {
        //             accum += reflection;
        //             println!("Perm # {} scored {}", perm_num, reflection);
        //             break 'seek_perm;
        //         }
        //         None => {}
        //     },
        // }
        // perm_num += 1;
    }
    final_answer(accum, submit, DAY, 2).await;
}

// fn debug_char_map(map: &Vec<Vec<char>>) {
//     for y in 0..map.len() {
//         for x in 0..map[0].len() {
//             print!("{}", map[y][x]);
//         }
//         println!();
//     }
// }

fn find_perm_score(permutations: &Vec<Vec<Vec<char>>>, original_score: usize) -> Option<usize> {
    for map in permutations {
        // debug_char_map(&map);
        // println!();
        let perm_score_result = score_map_avoiding(&map, original_score);
        match perm_score_result {
            Some(perm_score) => {
                if perm_score != original_score {
                    return Some(perm_score);
                }
            }
            None => {}
        }
    }

    None

    // panic!("I don't think this should happen");

    // original_score
}

// pub async fn d13s1(submit: bool, example: bool) {
//     let input = input(example).await;
//     let mut accum: usize = 0;
//     for map in input {
//         match get_v_reflection(&map) {
//             Some(reflection) => accum += (reflection + 1) * 100,
//             None => match get_h_reflection(&map) {
//                 Some(reflection) => accum += reflection + 1,
//                 None => {
//                     debug_map(&map);
//                     panic!("NO REFLECTIONS");
//                 }
//             },
//         }
//         println!("+++++ACCUM:{}+++++", accum);
//     }
//     final_answer(accum, submit, DAY, 1).await;
// }

fn debug_map(map: &Vec<String>) {
    for y in 0..map.len() {
        println!("{}", map[y]);
    }
}

// fn get_v_reflection(map: &Vec<Vec<char>>) -> Option<usize> {
//     for x in 0..map[0].len() - 3 {
//         if check_v_reflection(map, x) {
//             return Some(x);
//         }
//     }

//     None
// }

// fn get_h_reflection(map: &Vec<Vec<char>>) -> Option<usize> {
//     for y in 0..map.len() {
//         if check_h_reflection(map, y) {
//             return Some(y);
//         }
//     }

//     None
// }

// // fn check_v_reflection(map: &Vec<Vec<char>>, split_after: usize) -> bool {
// //     println!("\nCHECKING V SPLIT {}", split_after);
// //     let lower_y_limit = if split_after * 2 < map[0].len() {
// //         0
// //     } else {
// //         map[0].len() - split_after
// //     };
// //     let upper_y_limit = if split_after * 2 < map[0].len() {
// //         split_after * 2 + 1
// //     } else {
// //         map[0].len() - 1
// //     };

// //     for y in lower_y_limit..=split_after {
// //         for x in 0..map[0].len() {
// //             println!("[{}][{}] vs [{}][{}]", y, x, upper_y_limit - y, x,);
// //             println!("{} vs {}", map[y][x], map[upper_y_limit - y][x]);
// //             if map[y][x] != map[upper_y_limit - y][x] {
// //                 println!("FALSE");
// //                 return false;
// //             }
// //         }
// //     }

// //     true
// // }
// fn check_v_reflection(map: &Vec<Vec<char>>, split_after: usize) -> bool {
//     // println!("\nCHECKING V SPLIT {}", split_after);
//     let mut y0 = split_after;
//     let mut y1 = split_after + 1;

//     while y0 != usize::MAX && y1 < map.len() {
//         for x in 0..map[0].len() {
//             // println!("[{}][{}] vs [{}][{}]", y0, x, y1, x,);
//             // println!("{} vs {}", map[y0][x], map[y1][x]);
//             if map[y0][x] != map[y1][x] {
//                 // println!("FALSE");
//                 return false;
//             }
//         }
//         y0 -= 1;
//         y1 += 1;
//     }

//     true
// }
// fn check_h_reflection(map: &Vec<Vec<char>>, split_after: usize) -> bool {
//     // println!("\nCHECKING H SPLIT {}", split_after);
//     let mut x0 = split_after;
//     let mut x1 = split_after + 1;

//     while x0 != usize::MAX && x1 < map[0].len() {
//         for y in 0..map.len() {
//             // println!("[{}][{}] vs [{}][{}]", y, x0, y, x1,);
//             // println!("{} vs {}", map[y][x0], map[y][x1]);
//             if map[y][x0] != map[y][x1] {
//                 // println!("FALSE");
//                 return false;
//             }
//         }
//         x0 -= 1;
//         x1 += 1;
//     }

//     true
// }

// // fn check_h_reflection(map: &Vec<Vec<char>>, split_after: usize) -> bool {
// //     println!("\nCHECKING H SPLIT {}", split_after);
// //     let x_limit = split_after * 2 + 1;

// //     for y in 0..=split_after {
// //         for x in 0..map[0].len() {
// //             println!("[{}][{}] vs [{}][{}]", y, x, y, x_limit - x,);
// //             println!("{} vs {}", map[y][x], map[y][x_limit - x]);
// //             if map[y][x] != map[y][x_limit - x] {
// //                 println!("FALSE");
// //                 return false;
// //             }
// //         }
// //     }

// //     true
// // }
