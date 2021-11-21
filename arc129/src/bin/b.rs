use proconio::*;

#[fastout]
fn main() {
    input!{
        n: usize,
        lr: [(i64, i64); n]
    }
    let mut min = 0;
    let mut max = 1000000000;
    let mut ans = 0;
    let mut a:Vec<i64> = vec![max; max];
    for i in 0..n {
        let l = lr[i].0;
        let r = lr[i].1;
        if r < min {
            
        } else if l > max {
        } else {
            if l > min { min = l; }
            if r < max { max = r; }
            println!("{}", ans);
        } 
    }
}
