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
        _n: usize,
        s: String
    }
    let mut left_queue = VecDeque::new();
    let mut right_queue = VecDeque::new();
    let s_vec:Vec<char> = s.chars().collect();
    for (i, &v) in s_vec.iter().enumerate() {
        let idx = i+1;
        if v == 'L' {
            right_queue.push_front(idx-1);
        } else {
            left_queue.push_back(idx-1);
        }
    }
    if s_vec[s_vec.len()-1] == 'L' {
        left_queue.push_back(s_vec.len());
    } else {
        right_queue.push_front(s_vec.len());
    }
    for l in left_queue.iter() {
        print!("{} ", l);
    } 
    for r in right_queue.iter() {
        print!("{} ", r);
    } 
}
