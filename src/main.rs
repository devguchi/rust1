// use proconio::input;
// use text_io::read;
// use whiteread::parse_line;

fn main() {
    let mut v = vec![1, 10, 30, 2, 5, 1, 3, 3, 3, 3, 4, 230, 5, 43, 23];
    v.sort();
    println!("{:?}", v);
    let s = 3;
    // 同じ値の場合返すindexはバラバラになりうる
    let ans1 = v.binary_search(&s);
    println!("{:?}", ans1);
    let ans2 = lower_bound(&v, &s);
    println!("{:?}", ans2);
    let ans3 = upper_bound(&v, &s);
    println!("{:?}", ans3);
}

// 探している値の一番小さいindexを返す
fn lower_bound(v:&Vec<i64>, s:&i64) -> Result<usize, usize> {
    let mut low = 0;
    let mut high = v.len();
    let mut exist = false;
    while low != high {
        let mid = (low + high)/2;
        match v[mid].cmp(s) {
            std::cmp::Ordering::Less => {
                low = mid+1;
            },
            std::cmp::Ordering::Greater => {
                high = mid;
            },
            std::cmp::Ordering::Equal => {
                high = mid;
                exist = true;
            }
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
        match v[mid].cmp(s) {
            std::cmp::Ordering::Greater => {
                high = mid;
            },
            std::cmp::Ordering::Less => {
                low = mid+1;
            },
            std::cmp::Ordering::Equal => {
                low = mid+1;
                exist = true;
            }
        }
    }
    if exist { Ok(low-1) } else { Err(low) }
}