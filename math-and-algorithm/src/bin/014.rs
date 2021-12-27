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
        mut n: u64,
    }
    let mut factors = list_factors(n);
    factors.pop();
    factors = factors[1..].to_vec();
    factors = factors
        .iter()
        .cloned()
        .filter(|x| is_prime_number(*x as usize))
        .collect();
    let mut ans = vec![];
    let mut i = 0;
    while n > 1 {
        for j in i..factors.len() {
            if n % factors[j] == 0 {
                n = n / factors[j];
                ans.push(factors[j]);
                break;
            } else {
                i = j + 1;
            }
        }
    }
    for i in ans.iter() {
        print!("{} ", i)
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

fn is_prime_number(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
