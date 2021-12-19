use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! { s: Chars }
    let a: u32 = s[0] as u32 - '0' as u32;
    let b: u32 = s[2] as u32 - '0' as u32;
    println!("{}", a * b);
}
