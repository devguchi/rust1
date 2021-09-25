use std::io;

fn main() {
    let n = get_input_i64()[0];
    let mut k:u32 = 1;
    let base:i64 = 2;
    loop {
        let num = base.pow(k);
        if num > n {
            println!("{}", k-1);
            std::process::exit(0);
        }
        k += 1;
    }
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

fn get_input_i64() -> Vec<i64> {
    let words = get_input();
    words.iter().map(|word| word.parse().unwrap()).collect()
}


