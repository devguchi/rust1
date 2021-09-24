use std::io;

fn main() {
    let num = get_input_i64();
    let v = num[0] as f64;
    let t = num[1] as f64;
    let s = num[2] as f64;
    let d = num[3] as f64;
    let td = v*t;
    let sd = v*s;
    if d < td || sd < d {
        println!("Yes");
    } else {
        println!("No");
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


