use super::final_answer;
use super::input_raw;

use mlua::Function;

const DAY: u8 = 1;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw.lines().map(|item| item.to_owned()).collect();

    lines
}

pub async fn d01s1(submit: bool, example: bool) {
    let input = input(example).await;

    let lua = unsafe { mlua::Lua::unsafe_new() };
    let solver = lua
        .load(include_str!("../solutions_lua/runner.lua"))
        .eval::<Function>()
        .expect("Failed to load solver");
    let answer = solver.call::<u64>((DAY, 1, input)).expect("Solver failed");

    final_answer(answer, submit, DAY, 1).await;
}

pub async fn d01s2(submit: bool, example: bool) {
    let _input = input(example).await;

    let lua = mlua::Lua::new();
    let answer = lua
        .load(mlua::chunk! {
        1+1
                })
        .eval::<u64>()
        .expect("Failed to create Lua handler");

    final_answer(answer, submit, DAY, 2).await;
}
