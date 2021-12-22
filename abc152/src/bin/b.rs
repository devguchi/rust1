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
        a: usize,
        b: usize,
    }
    let mut num = a;
    let mut count = b;
    if a > b {
        num = b;
        count = a;
    }
    for _ in 0..count {
        print!("{}", num);
    }
}
