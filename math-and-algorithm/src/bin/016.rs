#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::{iproduct, Itertools};
use petgraph::{algo::is_isomorphic, graph::UnGraph};
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use num::integer::gcd;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let ans = a[1..].iter().fold(a[0], |ans, &x| gcd(ans, x));
    println!("{}", ans);
}

