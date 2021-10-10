// use proconio::input;
// use text_io::read;
use whiteread::parse_line;

fn main() {
    let x:Vec<i32> = parse_line().unwrap(); 
    let idx = x.iter().position(|&n| n == 0).unwrap();
    println!("{}", idx+1);
}

