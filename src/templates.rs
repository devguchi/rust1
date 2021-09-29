use std::io;
use std::collections::HashSet;

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

fn get_input_i64() -> Vec<i64> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.parse().unwrap()).collect()
}

fn get_input_lines(line_len:u32) -> Vec<String> {
    let mut vec:Vec<String> = vec![];
    let mut input:Vec<String>;
    for _ in 0..line_len {
        input = get_input();
        vec.append(&mut input);
    }
    vec
}

fn get_input_i64_lines(line_len:u32) -> Vec<i64> {
    let mut vec:Vec<i64> = vec![];
    let mut input:Vec<i64>;
    for _ in 0..line_len {
        input = get_input_i64();
        vec.append(&mut input);
    }
    vec
}

// Vec<String> -> HashSet<String>
fn vec_string_hashset(words:&Vec<String>) -> HashSet<String> {
    words.clone().into_iter().collect()
}

// Vec<String> -> HashSet<char>
fn string_hashset_char(s:&String) -> HashSet<char> {
    let vec:Vec<char> = s.clone().chars().collect();
    vec.into_iter().collect()
}

// 各桁の合計
fn digits_sum(s:&String) -> u32 {
    s.chars().map(|c| c.to_digit(10).unwrap()).sum()
}

// 各桁をu32のvecにする
fn string_vec_u32(s:&String) -> Vec<u32> {
    s.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

// a-zのcharのvec
fn a_z() -> Vec<char> {
    (b'a'..=b'z').map(|b| b as char).collect()
}

// 最小値
fn cmin(a:i64, b:i64) -> i64 {
    if a > b { b } else { a }
}

// 最大値
fn cmax(a:i64, b:i64) -> i64 {
    if a > b { a } else { b }
}

// 階乗（再帰）
fn fact(n:i64) -> i64 {
    if n < 1 {
        1
    } else {
        fact(n-1) * n
    }
}

// 階乗（DP）
fn fact(n:usize) -> i64 {
    let mut memo: [i64; 1000] = [1; 1000];
    _fact(n, &mut memo)
}

fn _fact(n:usize, memo: &mut [i64; 1000]) -> i64 {
    if n < 2 || memo[n] > 1 {
        memo[n]
    } else {
        memo[n] = _fact(n-1, memo) * (n as i64);
        memo[n]
    }
}

// 型名の表示
fn typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

