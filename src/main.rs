#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::{iproduct, Itertools};
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::{Add, Mul, Sub};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [f64; n],
        y: [f64; n]
    }
    let mut min_dist:f64 = 100000000.0;
    for (i, j) in (0..n).tuple_combinations() {
        let _dist = dist(x[i], y[i], x[j], y[j]);
        if min_dist > _dist { min_dist = _dist; }
    }
    println!("{}", min_dist);
}

fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)
}


