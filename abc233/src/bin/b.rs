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
        l: Usize1,
        r: Usize1,
        s: String
    }
    let s1 = &s[0..l];
    let s2 = &s[l..r + 1];
    let s3 = &s[r + 1..];
    let s2_2: String = s2.chars().rev().collect();
    println!("{}{}{}", s1, s2_2, s3);
}
