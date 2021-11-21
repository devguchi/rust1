use proconio::*;

#[fastout]
fn main() {
    input!{
        n: i64,
        k: usize,
        p_list: [(i64,i64,i64); n]
    }
    let mut a:Vec<i64> = p_list.iter().map(|&p| p.0+p.1+p.2).collect();
    a.sort();
    a.reverse();
    let limit_score = a[k-1];
    for p in p_list.iter() {
        let total = p.0+p.1+p.2+300;
        let ans = if total >= limit_score { "Yes" } else { "No" };
        println!("{}", ans);
    }
}
