#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::{iproduct, Itertools};
use petgraph::{algo::is_isomorphic, graph::UnGraph};
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! { s: String }
    let mut start = 0;
    for (i, v) in s.chars().enumerate() {
        if v != 'a' {break}
        start = i+1;
    } 
    if start == s.len() {
        println!("Yes");
    } else {
        let mut end = s.len();
        let c = s.chars().rev();
        for (i, v) in c.enumerate() {
            if v != 'a' {break}
            end = s.len() - i - 1;
        } 
        if start > s.len() - end {
            println!("No");
        } else {
            let v = s.chars().collect::<Vec<char>>();
            let t = &v[start..end];
            let mut ans = String::from("Yes");
            let n = t.len();
            for i in 0..n {
                if t[i] != t[n-1-i] {
                    ans = String::from("No");
                    break;
                }
            }
            println!("{}", ans);
        }
    }
}
