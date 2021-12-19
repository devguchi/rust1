use proconio::*;
// use proconio::marker::{Chars};
// use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize, k:usize,
        mut p: [usize; n]
    }
    let mut max_sum = 0;
    let mut max_idx = 0;
    for i in 0..n {
        if i + k > n { break; }
        let sum: usize = p[i..i + k].iter().sum();
        if sum > max_sum {max_sum = sum; max_idx = i;}
    }

    let mut ans = 0.0;
    for x in p[max_idx..max_idx+k].iter() {
        for y in 1..x+1 {
            ans += y as f64 / *x as f64;
        }
    }
    println!("{}", ans);
}
