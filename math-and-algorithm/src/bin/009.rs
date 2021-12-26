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
        s: usize,
        a: [usize; n]
    }
    let mut ans = String::from("No");
    for i in 0..2_usize.pow(n as u32) {
        let mut sum = 0;
        for j in 0..n {
            if i & (1 << j) > 0 {
                sum += a[j];
            };
        }
        if sum == s {
            ans = String::from("Yes");
        }
    }
    println!("{}", ans);
}
