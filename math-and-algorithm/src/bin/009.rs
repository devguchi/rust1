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
        s: usize,
        a_list: [usize; n]
    }
    let mut memo = vec![false; 10001];
    memo[0] = true;
    for a in a_list {
        for j in (a..=s).rev() {
            memo[j] |= memo[j - a];
        }
    }
    println!("{}", if memo[s] { "Yes" } else { "No" });
}
