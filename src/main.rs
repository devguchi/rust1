use std::io;

fn main() {
    let nxt = get_input_i64();
    let n = nxt[0] as f64;
    let x = nxt[1] as f64;
    let t = nxt[2] as f64;
    let ans = (n/x).ceil()*t;
    println!("{}", ans);
}

fn get_input_i64() -> Vec<i64> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.parse().unwrap()).collect()
}
