use proconio::input;

fn main() {
    input! {
        n: u32,
    }
    for c in n.to_string().chars() {
        if c == '7' {
            println!("Yes");
            std::process::exit(0);
        }
    }
    println!("No");
}

