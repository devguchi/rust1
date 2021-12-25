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
        x: f64,
        y: f64
    }
    let z = (y - x) / 10.0;
    let mut ans = z.ceil();
    if ans <= 0.0 {
        ans = 0.0;
    }
    println!("{}", ans as usize);
}
