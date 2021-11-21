use proconio::*;

#[fastout]
fn main() {
    input! {
        n: i64, m: usize,
        mut a: [i64; n]
    }
    let total: i64 = a.iter().sum();
    let limit = (total as f64) / (4.0 * (m as f64));
    a.sort();
    a.reverse();
    let ans = if (a[m - 1] as f64) >= limit { "Yes" } else { "No" };
    println!("{}", ans);
}
