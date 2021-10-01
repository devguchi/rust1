use std::io;

struct Mountain {
    name: String,
    height: i64
}

fn main() {
    let n = get_input()[0].parse().unwrap();
    let mut mountains: Vec<Mountain> = vec![];
    for _ in 0..n {
        let m = get_input();
        mountains.push(Mountain {
            name: m[0].clone(),
            height: m[1].parse().unwrap()
        });
    }
    mountains.sort_by(|a,b| b.height.cmp(&a.height));
    println!("{}", mountains[1].name);
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

