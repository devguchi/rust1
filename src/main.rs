fn main() {
    println!("Hello, world!");
    println!("{}", calc(12, 23));
    println!("{}", hello("hoge"));
}

fn calc(x: i64, y:i64) -> i64 {
    x + y
}

fn hello(s: &str) -> String {
    println!("{}", s);
    let s2: String = s.to_string();
    println!("{}", s2);
    s2+"HOGE"
}
