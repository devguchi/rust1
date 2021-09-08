fn main() {
    let mut a = 1;
    let b = loop {
        a += 1;
        if a > 100 {
            break a * 2;
        }
    };
    println!("{}", b);
}

