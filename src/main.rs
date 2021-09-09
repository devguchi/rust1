fn main() {
    let mut a = 1;
    let b = loop {
        a += 1;
        if a > 100 {
            break a * 2;
        }
    };
    println!("{}", b);
    let s = format!("{} {}", "1", "2");
    println!("{}", s);
    print!("hoge");
    print!("hoge2");
    eprint!("error {}", "ooo");
    println!("hoge2");
    let s = [1,2];
    dbg!(s);
    println!("file is {}", file!());
    println!("line is {}", line!());
}

