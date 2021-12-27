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
        a: [usize; n]
    }
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                for o in k + 1..n {
                    for p in o + 1..n {
                        if a[i] + a[j] + a[k] + a[o] + a[p] == 1000 {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
