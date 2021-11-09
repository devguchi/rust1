use proconio::input;
// use proconio::marker::Usize1;

fn main() {
    input!{
        n: u32,
        m: u32
    }
    println!("{}", n*(n-1)/2+m*(m-1)/2);
}
