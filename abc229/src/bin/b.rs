use proconio::*;

#[fastout]
fn main() {
    input!{
        a: String,
        b: String
    }
    let mut av:Vec<u32> = a.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut bv:Vec<u32> = b.chars().map(|c| c.to_digit(10).unwrap()).collect();
    av.reverse();
    bv.reverse();
    let vlen = if av.len() > bv.len() {bv.len()} else {av.len()};
    let mut ans = "Easy";
    for i in 0..vlen {
        if av[i] + bv[i] > 9 {
            ans = "Hard";
            break;
        }
    }
    println!("{}", ans);
}