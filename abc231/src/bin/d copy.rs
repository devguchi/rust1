use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input!{
        _n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }
    let mut hashmap = HashMap::new();
    for i in 0..m {
        let l = ab[i].0;
        let r = ab[i].1;
        let mut l_obj = hashmap.entry(l).or_insert(vec![0, 0]).clone();
        let mut r_obj = hashmap.entry(r).or_insert(vec![0, 0]).clone();
        if l_obj[0] != 0 && l_obj[1] != 0 {
            println!("No");
            std::process::exit(0);
        }
        if r_obj[0] != 0 && r_obj[1] != 0 {
            println!("No");
            std::process::exit(0);
        }
       if l_obj[0] == 0 && r_obj[1] == 0 {
            l_obj[0] = l;
            r_obj[1] = r;
            hashmap.insert(l, l_obj);
            hashmap.insert(r, r_obj);
        } else if l_obj[1] == 0 && r_obj[0] == 0 {
            l_obj[1] = l;
            r_obj[0] = r;
            hashmap.insert(l, l_obj);
            hashmap.insert(r, r_obj);
        } else {
            println!("No");
            std::process::exit(0);
        }
    }
    println!("Yes");
}
