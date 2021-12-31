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
        w: usize,
        wv: [(usize,usize); n]
    }
    let mut dp = vec![0; w + 1];
    for x in wv.iter() {
        for d in (x.0..=w).rev() {
            if dp[d - x.0] > 0 || d - x.0 == 0 {
                dp[d] = max(dp[d], dp[d - x.0] + x.1)
            }
        }
    }
    println!("{}", dp.iter().max().unwrap());
}
