use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); m]
    }

    let mut xab = vec![vec![false; n]; n];
    let mut xcd = vec![vec![false; n]; n];

    for i in 0..n {
        let a = ab[i].0 - 1;
        let b = ab[i].1 - 1;
        xab[a][b] = true;
        xab[b][a] = true;
    }
    for i in 0..n {
        let c = cd[i].0 - 1;
        let d = cd[i].1 - 1;
        xcd[c][d] = true;
        xcd[d][c] = true;
    }

    let mut perm = (0..n).collect_vec();
}
