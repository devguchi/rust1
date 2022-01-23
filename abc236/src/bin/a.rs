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
        s: String,
        a: Usize1,
        b: Usize1
    }
    let mut v: Vec<char> = s.chars().collect();
    let t = v[a];
    v[a] = v[b];
    v[b] = t;
    let s2: String = v.into_iter().collect();
    println!("{}", s2);
}
