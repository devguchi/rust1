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
        m: usize,
        ab: [(Usize1, Usize1); m],
        cd: [(Usize1, Usize1); m]
    }
    let mut xab = vec![vec![false; n]; n];
    let mut xcd = vec![vec![false; n]; n];
    for &(a, b) in ab.iter() {
        xab[a][b] = true;
        xab[b][a] = true;
    }
    for &(c, d) in cd.iter() {
        xcd[c][d] = true;
        xcd[d][c] = true;
    }
    let ans = (0..n)
        .permutations(n)
        .any(|t| iproduct!(0..n, 0..n).all(|(x, y)| xab[x][y] == xcd[t[x]][t[y]]));
    println!("{}", if ans { "Yes" } else { "No" });
}
