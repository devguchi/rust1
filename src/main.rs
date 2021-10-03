use std::io;

fn main() {
    let s = &get_input()[0];
    let len = s.len();
    let tail = &s[len-1..len];
    let mut ans = s.clone()+"s";
    if tail == "s" {
        ans = s.clone()+"es";
    }
    println!("{}", ans);
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

