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

    let mut ans = 0;
    let d = 998244353;
    for i in 1..19 {
        let min = 10_u64.pow(i - 1);
        let limit = 10_u64.pow(i);
        if n >= min {
            let num = if n >= limit { limit - min } else { n - min + 1 };
            if num % 2 == 0 {
                let b = (num/2)%d;
                let a = (1+num)%d;
                let sum = a*b;
                ans += sum % d;
            } else {
                let b = num%d;
                let a = ((1+num)/2)%d;
                let sum = a*b;
                ans += sum % d;
            }
        } else {
            break;
        }
    }
    println!("{}", ans % d);
}
