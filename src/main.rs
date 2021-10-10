use proconio::input;

fn main() {
    input! {
        s:char
    }
    match s {
        'A'..='Z' => println!("A"),
        _ => println!("a")
    }
}

