use proconio::*;

#[fastout]
fn main() {
    input!{
        n: usize, mut w:i64, 
        mut ab: [(i64,i64); n]
    }
    ab.sort_by(|a,b| b.0.cmp(&a.0));
    let mut ans = 0;
    for x in ab {
        if w - x.1 > 0 {
            w -= x.1;
            ans += x.0*x.1;
        } else {
            ans += x.0*w;
            break;
        }
    } 
    println!("{}", ans);
}
