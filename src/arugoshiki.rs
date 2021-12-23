use std::io;
use std::str::FromStr;

fn get_input<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();
    buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().ok().unwrap())
        .collect()
}

fn main() {
	let _n:Vec<usize> = get_input();
	let a:Vec<i64> = get_input();
	let count = a.iter().filter(|x| **x > 0).count();
	println!("{}", count);
}