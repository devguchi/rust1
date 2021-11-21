use proconio::*;

#[fastout]
fn main() {
    input! {
        mut n: i64,
        k:i64
    }
    n = n % k;
    let n2 = (n - k).abs();
    println!("{}", if n < n2 { n } else { n2 });
}
