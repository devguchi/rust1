use std::io;

fn main() {
    let n = get_input_i64();
    let s = n[0];
    let t = n[1];
    let mut ans = 0;
    for a in 0..101 {
        for b in 0..101 {
            for c in 0..101 {
                if a+b+c <= s && a*b*c <= t {
                    ans += 1;
                }
            }
        }
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
