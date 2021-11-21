use proconio::*;

#[fastout]
fn main() {
    input!{ r: f64, }
    let ans = r*2.0*std::f64::consts::PI;
    println!("{}", ans);
}
