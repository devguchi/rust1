use proconio::*;

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut ans = 0;
    for i in 1..n + 1 {
        if i % 3 != 0 && i % 5 != 0 {
            ans += i;
        }
    }
    println!("{}", ans);
}
