use proconio::input;
// use proconio::marker::Usize1;
// use proconio::marker::Chars;
// use std::collections::HashSet;

fn main() {
    input!{
        n: f32,
    }
    println!("{}", (n/2.0).ceil());
}
