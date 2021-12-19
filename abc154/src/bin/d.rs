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
        let xf = *x as f64;
        ans += (1.0+xf)*xf/2.0/xf;
    }
    println!("{}", ans);
}

