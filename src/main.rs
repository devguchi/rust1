use proconio::input;
// use proconio::marker::Usize1;
// use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input!{
        n: [i32; 3]
    }
    let h:HashSet<i32> = n.into_iter().collect();
    let mut result = "No";
    if h.len() == 2 { result = "Yes"; }
    println!("{}", result);
}
