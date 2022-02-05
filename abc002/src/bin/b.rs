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
        w: String,
    }
    let mut new_w:Vec<char> = vec![];
    let targets:Vec<char> = vec!['a', 'i', 'u', 'e', 'o'];
    for c in w.chars() {
        if !targets.contains(&c) {
            new_w.push(c);
        }
    }
    let ans:String = new_w.iter().collect();
    println!("{}", ans);
}
