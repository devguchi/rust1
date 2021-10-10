use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [u64; n],
        mut b: [u64; n],
    }
    a.sort();
    b.sort();
    let mut l = 0;
    let mut r = 1_000_000_000_000_000_000;
    while r - l > 1 {
        let m = (l + r) / 2;
        let mut cnt = 0;
        let mut j = 0;
        for a in a.iter().rev() {
            while j < b.len() && *a * b[j] <= m {
                j += 1;
            }
            cnt += j;
        }
        if k <= cnt {
            r = m;
        } else {
            l = m;
        }
    }
    println!("{}", r);
}
