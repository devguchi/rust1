#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::{iproduct, Itertools};
use petgraph::{algo::is_isomorphic, graph::UnGraph};
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut cnt: Vec<usize> = vec![0; 4];
    for i in a.iter() {
        cnt[i / 100 - 1] += 1;
    }
    println!("{}", cnt[0] * cnt[3] + cnt[1] * cnt[2]);
}
