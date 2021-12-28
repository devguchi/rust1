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
        h: [i64; n]
    }
    let mut memo: Vec<i64> = vec![10000; n];
    memo[0] = 0;
    for i in 0..n {
        if i > 0 {
            let cost1 = (h[i] - h[i - 1]).abs();
            memo[i] = memo[i - 1] + cost1;
        }
        if i > 1 {
            let cost2 = (h[i] - h[i - 2]).abs();
            memo[i] = min(memo[i], memo[i - 2] + cost2);
        }
    }
    println!("{}", memo[n - 1]);
}
