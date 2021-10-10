use proconio::input;
use whiteread::parse_line;

fn main() {
    input! { n:usize, k:i64 }
    let a:Vec<i64> = parse_line().unwrap(); 
    let idx = lower_bound(&a, &k);
    println!("{} {}", a.len(), idx);
    if a[n-1] < k {
        println!("-1");
    } else { 
        println!("{}", idx);
    }
}

fn lower_bound(v:&Vec<i64>, s:&i64) -> usize {
    let mut low = 0;
    let mut high = v.len();
    while low != high {
        let mid = (low + high)/2;
        if v[mid] > *s {
            low = mid+1;
        } else { 
            high = mid;
        }
    }
    low
}
