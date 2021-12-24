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
        m:usize,
        ps: [(Usize1, String); m]
    }
    let mut ac = vec![0; n];
    let mut wa = vec![0; n];
    let mut ac_cnt = 0;
    let mut wa_cnt = 0;
    for x in ps.iter() {
        if x.1 == String::from("AC") {
            if ac[x.0] == 0 {
                ac_cnt += 1;
                ac[x.0] = 1;
                wa_cnt += wa[x.0];
            }
        } else {
            if ac[x.0] == 0 {
                wa[x.0] += 1;
            }
        }
    }
    println!("{} {}", ac_cnt, wa_cnt);
}
