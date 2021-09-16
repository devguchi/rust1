use std::io;

fn main() {
    let _ = get_input();
    let words = get_input();
    let mut a_vec:Vec<u32> = words.iter()
        .map(|v| v.parse().unwrap()).collect();
    a_vec.sort_by(|a,b| b.cmp(a));
    let mut alice = 0;
    let mut bob = 0;
    for (i, a) in a_vec.iter().enumerate() {
        if i%2 == 0 {
            alice += a;
        } else {
            bob += a;
        }
    }
    let diff = alice - bob;
    println!("{}", diff);
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}


