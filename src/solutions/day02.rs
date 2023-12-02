use super::final_answer;
use super::input_raw;

const DAY: u8 = 2;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw.split("\n").map(|item| item.to_owned()).collect();

    lines
}

pub async fn d02s1(submit: bool, example: bool) {
    let input = input(example).await;
    final_answer(input[0].to_owned(), submit, DAY, 1).await;
}

pub async fn d02s2(submit: bool, example: bool) {
    let input = input(example).await;
    final_answer(input[0].to_owned(), submit, DAY, 2).await;
}
