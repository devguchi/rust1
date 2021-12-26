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
        s: usize
    }
    let mut ans = 0;
    for (x, y) in iproduct!(1..n + 1, 1..n + 1) {
        if x + y <= s {
            ans += 1
        }
    }
    println!("{}", ans);
}
