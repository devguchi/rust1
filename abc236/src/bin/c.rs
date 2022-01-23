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
        s: [String; n],
        mut t: [String; m]
    }
    let mut ans = vec!["No".to_string(); n];
    t.sort();
    for (i, s) in s.iter().enumerate() {
        let idx = lower_bound(&t, s.clone());
        if idx >= 0 && idx < m && t[idx] == *s {
            ans[i] = "Yes".to_string();
        }
    }
    for a in ans.iter() {
        println!("{}", a);
    }
}

fn lower_bound(v: &Vec<String>, s: String) -> usize {
    let mut left = 0;
    let mut right = v.len();
    while left != right {
        let mid = (left + right) / 2;
        if s > v[mid] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}