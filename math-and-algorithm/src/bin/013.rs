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
        n: u64,
    }
    let v = list_factors(n);
    for i in v.iter() {
        println!("{}", i);
    }
}

fn list_factors(n: u64) -> Vec<u64> {
    if n <= 0 {
        return vec![];
    }
    let mut ans = vec![];
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            ans.push(i);
            if i != n && i != n / i {
                ans.push(n / i)
            }
        }
        i += 1;
    }
    ans.sort();
    ans
}
