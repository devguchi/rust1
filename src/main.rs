fn main() {
    println!("{}", fact(3));
}

fn fact(n:i64) -> i64 {
    if n < 1 {
        1
    } else {
        fact(n-1) * n
    }
}

