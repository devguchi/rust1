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
        a_list: [usize; n]
    }
    let mut dp = vec![0; n];
    for (i, a) in a_list.iter().enumerate() {
        if i < 2 {
            dp[i] = *a;
        } else {
            dp[i] = max(dp[i - 1], a + dp[i - 2]);
        }
    }
    println!("{}", dp.pop().unwrap());
}
