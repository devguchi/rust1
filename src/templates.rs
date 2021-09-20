use std::io;

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

fn get_input_lines(line_len:u32) -> Vec<String> {
    let mut vec:Vec<String> = vec![];
    let mut input:Vec<String>;
    for _ in 0..line_len {
        input = get_input();
        vec.append(&mut input);
    }
    vec
}

fn get_input_i64() -> Vec<i64> {
    let words = get_input();
    words.iter().map(|word| word.parse().unwrap()).collect()
}

