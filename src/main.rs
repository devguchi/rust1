use std::io;

fn main() {
    let nv = get_input_i64();
    let v = nv[1];
    let a_list = get_input_i64();
    let mut ans = "No";
    for a in a_list {
        if a == v {
            ans = "Yes";
            break;
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