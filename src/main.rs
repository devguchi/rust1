use std::io;

fn main() {
    let num = get_input_i64();
    let _ = num[0];
    let x = num[1];
    let a = get_input_i64();
    let mut total = 0;
    let mut ans = "No";
    for (i, val) in a.iter().enumerate() {
        total += *val;
        if (i+1)%2 == 0 {
            total -= 1;
        }
    }
    if total <= x {
        ans = "Yes";
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

