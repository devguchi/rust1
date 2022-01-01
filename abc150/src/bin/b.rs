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
        s: String
    }
    let mut ans = 0;
    for i in 0..n - 2 {
        if &s[i..i + 3] == String::from("ABC") {
            ans += 1;
        }
    }
    println!("{}", ans);
}
