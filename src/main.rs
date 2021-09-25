use std::io;

fn main() {
    let words = get_input_lines(4);
    let t:Vec<char> = words[3].chars().collect();
    for tc in t.iter() {
        let i = *tc as usize - 48 - 1;
        print!("{}", &words[i]);
    }
}

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

