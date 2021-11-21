use proconio::*;

#[fastout]
fn main() {
    input!{
        q: usize,
        tx: [(u64, usize); q],
    }
    let n:usize = 2usize.pow(20);
    let mut a:Vec<i64> = vec![-1; n];
    for i in 0..q {
        let mut h = tx[i].1;
        let mut a_idx = h%n;
        if tx[i].0 == 1 {
            while a[a_idx] != -1 {
                h += 1;
                a_idx = h%n;
            }
            a[a_idx] = tx[i].1 as i64;
        } else {
            println!("{}", a[a_idx]);
        }
    }
}
