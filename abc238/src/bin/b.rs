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
        a: [usize; n],
    }
    let mut k: Vec<usize> = vec![0];
    for i in a.iter() {
        k = k
            .iter()
            .map(|r| if r + i >= 360 { r + i - 360 } else { r + i })
            .collect();
        k.push(0);
    }
    k.sort();
    let mut ans = 0;
    for i in 0..k.len() {
        let mut diff = 0;
        if i == k.len() - 1 {
            diff = 360 - k[i];
        } else {
            diff = k[i + 1] - k[i];
        }
        if ans < diff {
            ans = diff
        }
    }
    println!("{}", ans);
}
