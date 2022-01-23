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
        a: [usize; 4*n-1]
    }
    let mut d = vec![4; n];
    for a in a.iter() {
        d[a - 1] -= 1;
    }
    let ans = d.iter().position(|&x| x > 0).unwrap() + 1;
    println!("{}", ans);
}
