use proconio::*;

#[fastout]
fn main() {
    input! {
        n: i32, m: usize,
        a: [i32; m]
    }
    let sum: i32 = a.iter().sum();
    let mut ans = n - sum;
    if ans < 0 {
        ans = -1;
    }
    println!("{}", ans);
}
