use proconio::input;
// use proconio::marker::Usize1;
// use proconio::marker::Chars;
// use std::collections::HashSet;

fn main() {
    input!{
        n: i64,
        k: usize,
        p_list: [(i64,i64,i64); n]
    }
    let mut a: Vec<i64> = vec![];
    for p in p_list.iter() {
        let total = p.0+p.1+p.2;
        a.push(total);
    }
    a.sort_by(|x,y| y.cmp(x));
    let limit_score = a[k-1];
    for p in p_list.iter() {
        let total = p.0+p.1+p.2+300;
        if total >= limit_score {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
