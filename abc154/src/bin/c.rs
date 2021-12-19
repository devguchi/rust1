use proconio::*;
// use proconio::marker::{Chars};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let hashset: HashSet<usize> = a.into_iter().collect();
    let ans = if hashset.len() < n { "NO" } else { "YES" };
    println!("{}", ans);
}
