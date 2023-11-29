use super::final_answer;
use super::input_raw;

const DAY: u8 = 0;

async fn input() -> Vec<String> {
    let raw = input_raw(DAY).await;
    let lines = raw.split("\n").map(|item| item.to_owned()).collect();

    lines
}

pub async fn d00s1(submit: bool) {
    let _input = input().await;
    final_answer("NaN", submit, DAY, 1).await;
}

pub async fn d00s2(submit: bool) {
    let _input = input().await;
    final_answer("NaN", submit, DAY, 2).await;
}
