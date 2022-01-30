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
        h: usize,
        w: usize,
        a: [[usize; w]; h]
    }
    for i in 0..w {
        for j in 0..h {
            print!("{} ", a[j][i]);
        }
    }
}
