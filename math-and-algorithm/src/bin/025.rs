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
        a: [f64; n],
        b: [f64; n]
    }
    let ans: f64 = (0..n).fold(0.0, |ans, i| ans + a[i] * 1.0 / 3.0 + b[i] * 2.0 / 3.0);
    println!("{}", ans);
}
