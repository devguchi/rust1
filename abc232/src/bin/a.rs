use proconio::*;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{ s: Chars }
    let a:u32 = s[0] as u32 - '0' as u32;
    let b:u32 = s[2] as u32 - '0' as u32;
    println!("{}", a*b);
}

