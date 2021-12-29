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
        mut a: [usize; n]
    }
    merge_sort(&mut a);
    let ans = a.iter().join(" ");
    println!("{}", ans);
}

fn _merge(a: Vec<usize>, b: Vec<usize>) -> Vec<usize> {
    let mut c: Vec<usize> = vec![];
    let mut a_idx = 0;
    let mut b_idx = 0;
    loop {
        if a_idx >= a.len() && b_idx >= b.len() {
            break;
        } else if a_idx >= a.len() {
            c.push(b[b_idx]);
            b_idx += 1;
        } else if b_idx >= b.len() {
            c.push(a[a_idx]);
            a_idx += 1;
        } else if a[a_idx] > b[b_idx] {
            c.push(b[b_idx]);
            b_idx += 1;
        } else if b[b_idx] >= a[a_idx] {
            c.push(a[a_idx]);
            a_idx += 1;
        }
    }
    c
}
fn merge_sort(v: &mut Vec<usize>) {
    if v.len() == 1 {
        return;
    }
    let m = ((v.len() as f64 / 2.0).floor()) as usize;
    let mut a: Vec<usize> = v[0..m].to_vec();
    let mut b: Vec<usize> = v[m..v.len()].to_vec();
    merge_sort(&mut a);
    merge_sort(&mut b);
    *v = _merge(a, b);
}
