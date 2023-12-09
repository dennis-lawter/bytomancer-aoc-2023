use super::final_answer;
use super::input_raw;

const DAY: u8 = 9;

async fn input(example: bool) -> Vec<Vec<i64>> {
    let raw = input_raw(DAY, example).await;
    let lines: Vec<String> = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    lines
        .iter()
        .map(|i| {
            i.split(" ")
                .map(|j| str::parse::<i64>(j).unwrap())
                .collect()
        })
        .collect()
}

pub async fn d09s1(submit: bool, example: bool) {
    let oasis = input(example).await;
    let mut accum: i64 = 0;
    for num_list in oasis {
        accum += p1_solver(&num_list);
    }
    final_answer(accum, submit, DAY, 1).await;
}

pub async fn d09s2(submit: bool, example: bool) {
    let oasis = input(example).await;
    let mut accum: i64 = 0;
    for num_list in oasis {
        accum += p2_solver(&num_list);
    }
    final_answer(accum, submit, DAY, 2).await;
}

fn all_values_zero(num_list: &Vec<i64>) -> bool {
    for num in num_list {
        if num != &0 {
            return false;
        }
    }
    return true;
}

fn p1_solver(num_list: &Vec<i64>) -> i64 {
    if all_values_zero(num_list) {
        return 0;
    }
    let mut new_list = Vec::with_capacity(num_list.len() - 1);
    for i in 1..num_list.len() {
        let diff = num_list[i] - num_list[i - 1];
        new_list.push(diff);
    }
    let val = p1_solver(&new_list);

    val + num_list[num_list.len() - 1]
}

fn p2_solver(num_list: &Vec<i64>) -> i64 {
    if all_values_zero(num_list) {
        return 0;
    }
    let mut new_list = Vec::with_capacity(num_list.len() - 1);
    for i in 1..num_list.len() {
        let diff = num_list[i] - num_list[i - 1];
        new_list.push(diff);
    }
    let val = p2_solver(&new_list);

    num_list[0] - val
}
