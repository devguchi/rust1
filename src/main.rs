use std::io;

fn main() {
    let n:u32 = get_input()[0].parse().unwrap();
    if  n < 40 {
        println!("{}", 40-n);
    } else if n < 70 {
        println!("{}", 70-n);
    } else if n < 90 {
        println!("{}", 90-n);
    } else {
        println!("expert");
    }
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}


