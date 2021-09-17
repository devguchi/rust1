use std::io;

fn main() {
    let words = get_input();
    let n:i64 = words[0].parse().unwrap();
    let y:i64 = words[1].parse().unwrap();
    for m in 0..n+1 {
        for g in 0..n+1-m {
            let s = n-m-g;
            if 10000*m+5000*g+1000*s == y {
                println!("{} {} {}", m,g,s);
                std::process::exit(0);
            }
        }
    }
    println!("-1 -1 -1");
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}


