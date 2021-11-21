use proconio::*;

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n-1]
    }
    let mut ans_list:Vec<usize> = vec![0; n];
    for i in 0..n-1 {
        ans_list[a[i]-1] += 1;
    }
    for ans in ans_list {
        println!("{}", ans);
    }
}
