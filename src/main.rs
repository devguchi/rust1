use proconio::input;
// use proconio::marker::Usize1;
// use proconio::marker::Chars;
// use std::collections::HashSet;

fn main() {
    input!{
        n: i32,
        r: i32
    }
    if n >= 10 {
        println!("{}", r)
    } else {
        println!("{}", r+100*(10-n));
    }
}
