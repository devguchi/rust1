use proconio::input;

fn main() {
    input! {
        mut n: String,
    }
    let ans = match n.pop().unwrap() as i64 - 48 {
        2 | 4 | 5 | 7 | 9 => "hon",
        0 | 1 | 6 | 8 => "pon",
        3 => "bon",
        _ => "_"
    };
    println!("{}", ans);
}
