#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::*;
use std::io;
use std::str::FromStr;

#[fastout]
fn main() {
    let nx:Vec<usize> = get_input();
    let n = nx[0];
    let x = nx[1] as u128;
    let tmp: Vec<Vec<u128>> = get_input_lines(n);
    let mut la: Vec<Vec<u128>> = vec![];
    for t in tmp.iter() { la.push(t[1..].to_vec()); }
    let mut a = la[0].clone();
    for b in 1..n {
        a = merge(a, la[b].clone());
    }
    let ans = a.iter().filter(|y| **y == x).count();
    println!("{}", ans);
}

fn merge(a: Vec<u128>, b: Vec<u128>) -> Vec<u128> {
    let mut result: Vec<u128> = vec![];
    for i in 0..a.len() {
        for j in 0..b.len() {
            result.push(a[i] * b[j]);
        }
    }
    result
}

fn get_input<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();
    buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().ok().unwrap())
        .collect()
}
fn get_input_lines<T: FromStr>(line_len: usize) -> Vec<Vec<T>> {
    let mut vec: Vec<Vec<T>> = vec![];
    for _ in 0..line_len {
        let v: Vec<T> = get_input();
        vec.push(v);
    }
    vec
}
