#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::marker::Usize1;
use proconio::*;
use std::io;
use std::str::FromStr;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: u128,
    }
    let mut la = vec![];
    for _ in 0..n {
        input! {
            l: usize,
            a: [u128; l]
        }
        la.push(a);
    }
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
