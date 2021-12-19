use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); m]
    }
    let mut a: Vec<u64> = vec![0; n];
    let mut b: Vec<u64> = vec![0; n];
    for i in 0..m {
        a[ab[i].0 - 1] += 1;
        a[ab[i].1 - 1] += 1;
        b[cd[i].0 - 1] += 1;
        b[cd[i].1 - 1] += 1;
    }
    a.sort();
    b.sort();
    let mut ans = if a == b { "Yes" } else { "No" };
    if a > b || b > a {
        ans = "No";
    }
    println!("{}", ans);
}
