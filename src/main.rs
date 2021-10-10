use proconio::input;
// use whiteread::parse_line;

fn main() {
    input! {
        n:usize,
        mut a:[i64; n],
        b:[i64; n],
        mut c:[i64; n],
    }
    a.sort();
    c.sort();
    let mut ans = 0;
    for b_idx in 0..n {
        let a_idx = lower_bound(&a, &b[b_idx]);
        let c_idx = upper_bound(&c, &b[b_idx]);
        let num = a_idx*(n-c_idx);
        ans += if num > 0 { num } else { 0 };
    } 
    println!("{}", ans);
}

fn lower_bound(v:&Vec<i64>, s:&i64) -> usize {
    let mut low = 0;
    let mut high = v.len();
    while low != high {
        let mid = (low + high)/2;
        if v[mid] < *s {
            low = mid+1;
        } else { 
            high = mid;
        }
    }
    low
}

fn upper_bound(v:&Vec<i64>, s:&i64) -> usize {
    let mut low = 0;
    let mut high = v.len();
    while low != high {
        let mid =  (low+high)/2;
        if v[mid] > *s {
            high = mid;
        } else { 
            low = mid+1;
        }
    }
    low
}

