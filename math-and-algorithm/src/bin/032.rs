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
        x: usize,
        mut a:[usize; n]
    }
    a.sort();
    let ans = match a.binary_search(&x) {
        Ok(_) => "Yes",
        _ => "No",
    };
    println!("{}", ans);
}
