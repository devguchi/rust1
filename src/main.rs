use proconio::input;
use std::mem;

fn main() {
    input! {
        mut a: u32,
        mut b: u32,
        mut c: u32,
    }
    mem::swap(&mut a, &mut b);
    mem::swap(&mut a, &mut c);
    println!("{} {} {}", a,b,c);
}
