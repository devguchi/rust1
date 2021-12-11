use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input!{
        n: usize,
        s: [String; n]
    }

    let mut hashmap = HashMap::new();
    for i in s {
        let cnt = hashmap.entry(i).or_insert(0);
        *cnt += 1;
    }
    let mut vec:Vec<_> = hashmap.into_iter().collect();
    vec.sort_by(|x,y| y.1.cmp(&x.1)); 
    let ans = vec[0].0.clone();
    println!("{}", ans);
}
