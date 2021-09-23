use std::io;

fn main() {
    let num = get_input_i64();
    let x = num[0] as f64;
    let mut ans = ((x/100.0).ceil()*100.0-x) as i64;
    if ans == 0 {
        ans = 100;
    }
    println!("{}", ans);
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


