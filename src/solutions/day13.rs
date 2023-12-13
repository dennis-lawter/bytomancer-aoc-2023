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
    // let mut normal_maps: Vec<Vec<String>> = vec![];
    // let mut rotated_maps: Vec<Vec<String>> = vec![];
    for map in input {
        let (normal, rotated) = get_rotations(&map);
        match get_tb_reflection(&normal) {
            Some(reflection) => accum += reflection * 100,
            None => match get_tb_reflection(&rotated) {
                Some(reflection) => accum += reflection,
                None => todo!(),
            },
        }
        // normal_maps.push(normal);
        // rotated_maps.push(rotated);
    }
    // for i in 0..normal_maps.len() {
    //     let normal =
    // }
    final_answer(accum, submit, DAY, 1).await;
}

fn get_tb_reflection(map: &Vec<String>) -> Option<usize> {
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
        return Some(y);
    }
    None
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

// fn debug_map(map: &Vec<Vec<char>>) {
//     for y in 0..map.len() {
//         for x in 0..map[0].len() {
//             print!("{}", map[y][x]);
//         }
//         println!();
//     }
// }

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

pub async fn d13s2(submit: bool, example: bool) {
    let input = input(example).await;
    let mut accum = 0;
    final_answer(accum, submit, DAY, 2).await;
}
