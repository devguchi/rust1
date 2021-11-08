use proconio::input;
// use proconio::marker::Usize1;

fn main() {
    input!{
        n: usize,
        a: [i64; n],
        b: [i64; n]
    }
    let max = a.iter().max().unwrap();
    let min = b.iter().min().unwrap();
    let mut cnt = min-max+1;
    if cnt <= 0 { cnt = 0;}
    println!("{}", cnt);
}
