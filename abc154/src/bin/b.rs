use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        s: Chars
    }
    for _ in s {
        print!("x");
    }
}
