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
        p: [usize; n],
        q: [usize; n]
    }
    let v: Vec<Vec<usize>> = (1..n + 1).permutations(n).collect();
    let mut a = -1;
    let mut b = -1;
    for (i, x) in v.iter().enumerate() {
        if *x == p {
            a = (i + 1) as i64
        }
        if *x == q {
            b = (i + 1) as i64
        }
    }
    println!("{}", (a - b).abs());
}
