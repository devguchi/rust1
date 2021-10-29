use proconio::input;

fn main() {
    input! {
        mut n: i64,
        k: i32,
    }
    for _ in 0..k {
        if n % 200 == 0 {
            n /= 200;
        } else {
            n = (n.to_string()+"200").parse().unwrap();
        }
    }
    println!("{}", n);
}

