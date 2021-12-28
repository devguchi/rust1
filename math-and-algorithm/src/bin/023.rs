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
        b: [f64; n],
        r: [f64; n]
    }
    let bs = b.iter().sum::<f64>();
    let rs = r.iter().sum::<f64>();
    let n = n as f64;
    println!("{}", bs / n + rs / n);
}
