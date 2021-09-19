use std::io;

fn main() {
    let n:usize = get_input()[0].parse().unwrap();
    let s = &get_input()[0];
    let s_vec:Vec<char> = s.chars().collect();
    if s_vec[n-1] == 'o' {
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

