use proconio::input;
// use proconio::marker::Usize1;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input!{
        s: Chars,
    }
    let h:HashSet<char> = s.into_iter().collect();
    let mut result = "No";
    if h.len() > 1 {
        result = "Yes";
    }
    println!("{}", result);
}
