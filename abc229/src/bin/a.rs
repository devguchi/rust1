use proconio::*;

#[fastout]
fn main() {
    input!{
        s1: String,
        s2: String,
    }
    let c1:Vec<char> = s1.chars().collect();
    let c2:Vec<char> = s2.chars().collect();
    let mut ans = "Yes";
    if c1[0] != c1[1] && c2[0] != c2[1] && c1[0] != c2[0] {
        ans = "No";
    }
    println!("{}", ans);
}
