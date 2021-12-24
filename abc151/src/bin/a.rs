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
        c: char,
    }
    let az = a_z_vec_char();
    let idx = find_index(&az, c);
    println!("{}", az[idx + 1]);
}

fn a_z_vec_char() -> Vec<char> {
    (b'a'..=b'z').map(|b| b as char).collect()
}

fn find_index<T: PartialEq + Copy>(vec: &Vec<T>, search_target: T) -> usize {
    vec.iter().position(|&x| x == search_target).unwrap()
}
