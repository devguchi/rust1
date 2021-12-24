#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::{iproduct, Itertools};
use petgraph::{algo::is_isomorphic, graph::UnGraph};
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        a: [i32; n-1]
    }
    let sum: i32 = a.iter().sum();
    let x = (m * n) as i32 - sum;
    if x > k as i32 {
        println!("-1");
    } else if x <= 0 {
        println!("0");
    } else {
        println!("{}", x);
    };
}
