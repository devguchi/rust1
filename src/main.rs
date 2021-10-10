// use proconio::input;
// use whiteread::parse_line;

fn main() {
    let v = vec![1, 2, 3, 3, 3, 6, 9];
    let s = 100;
    println!("{:?}", lower_bound(&v, &s));
    println!("{:?}", upper_bound(&v, &s));
}

// 探している値の一番小さいindexを返す
fn lower_bound(v:&Vec<i64>, s:&i64) -> Result<usize, usize> {
    let mut low = 0;
    let mut high = v.len();
    let mut exist = false;
    while low != high {
        let mid = (low + high)/2;
        if v[mid] < *s {
            low = mid+1;
        } else if v[mid] > *s {
            high = mid;
        } else { 
            high = mid;
            exist = true;
        }
    }
    if exist { Ok(low) } else { Err(low) }
}

// 探している値の一番大きいindexを返す
fn upper_bound(v:&Vec<i64>, s:&i64) -> Result<usize, usize> {
    let mut low = 0;
    let mut high = v.len();
    let mut exist = false;
    while low != high {
        let mid =  (low+high)/2;
        if v[mid] > *s {
            high = mid;
        } else if v[mid] < *s {
            low = mid+1;
        } else { 
            low = mid+1;
            exist = true;
        }
    }
    if exist { Ok(low-1) } else { Err(low) }
}
