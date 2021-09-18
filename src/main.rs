use std::io;

fn main() {
    let s = &get_input()[0];
    let s_vec:Vec<char> = s.chars().rev().collect();
    let targets = ["dream", "dreamer", "erase", "eraser"];
    let mut targets_rev:Vec<String> = vec![]; 
    for target in &targets {
        targets_rev.push(target.to_string().chars().rev().collect());
    }
    let mut i = 0;
    'outer:loop {
        if i > s_vec.len()-1 {
            println!("YES");
            std::process::exit(0);
        }
        let mut s_str:String;
        for target in &targets_rev {
            if i+target.len() > s_vec.len() {
                continue;
            }
            s_str = s_vec[i..i+target.len()].into_iter().collect();
            if s_str == *target {
                i += target.len();
                continue 'outer;
            }
        }
        println!("NO");
        std::process::exit(0);
    }
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

