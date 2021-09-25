use std::io;

fn main() {
    let p = get_input_i64();
    let az:Vec<char> = a_z();
    for pi in p {
        let i = pi as usize;
        print!("{}", az[i-1]);
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

fn a_z() -> Vec<char> {
    (b'a'..=b'z').map(|b| b as char).collect()
}

