#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::{iproduct, Itertools};
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

// #[fastout]
fn main() {
    let b = 0b10010010;
    let c = b << 1;
    println!("{:b}", b);
    println!("{:b}", c);
    let d = b & b;
    let e = b | b;
    let f = b ^ b;
    let g = !b;
    let h = g << 1;
    println!("{:b}", d);
    println!("{:b}", e);
    println!("{:b}", f);
    println!("{:b}", g);
    println!("{:b}", h);
    println!("{}", format!("{:b}", h).len());

    for i in 0..8 {
        let mut ans = String::new();
        for j in 0..3 {
            ans += if i & (1 << j) > 0 { "o" } else { "x" };
        }
        println!("{:0>3b} => {}", i, ans.chars().rev().collect::<String>());
    }
}
