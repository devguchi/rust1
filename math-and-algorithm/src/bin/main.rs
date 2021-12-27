#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::{iproduct, Itertools};
use num::integer::gcd;
use petgraph::{algo::is_isomorphic, graph::UnGraph};
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    let a: Vec<usize> = vec![100, 150, 200, 250, 300];
    a.iter().combinations(5).fold(0, |ans, v| {
        dbg!(v.iter().cloned().sum::<usize>());
        // let hoge:usize = **v.iter().sum::<usize>();
        // hoge
        10
    });
    // println!("{:?}", sum);
    // let ans = a.iter().combinations(5).fold(0, |ans, v| if v.iter().sum::<usize>() == 1000 {ans+1} else {ans});
    // println!("{}", ans);

    let v = vec![24, 48];
    let ans = v[1..].iter().fold(v[0], |ans, &x| gcd(ans, x));
    println!("{}", ans);

    println!("usize: {}", std::usize::MAX);
    println!("u128: {}", std::u128::MAX);
    println!("f64: {:e}", std::f64::MAX);
    println!("u64: {}", std::u64::MAX);
    println!("u32: {}", std::u32::MAX);
    println!("u16: {}", std::u16::MAX);
    println!("u8: {}", std::u8::MAX);
    println!("isize: {}", std::isize::MAX);
    println!("i128: {}", std::i128::MAX);
    println!("i64: {}", std::i64::MAX);
    println!("i32: {}", std::i32::MAX);
    println!("i16: {}", std::i16::MAX);
    println!("i8: {}", std::i8::MAX);

    let len = std::u128::MAX
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>()
        .len();
    println!("len:{}", len);

    let len = std::u64::MAX
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>()
        .len();
    println!("len:{}", len);

    let len = std::u32::MAX
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>()
        .len();
    println!("len:{}", len);

    let hoge: f64 = std::f64::MAX;
    let hoge2: u64 = std::u64::MAX;
    let hoge3: f64 = hoge2 as f64;
    let hoge4: u128 = std::u128::MAX;
    let hoge5: f64 = hoge4 as f64;
    println!("{}", hoge3.log10());
    println!("{}", 1000.1_f64.log10());
    println!("{}", hoge5.log10());
    println!("{}", 1024.0_f64.sqrt());
}
