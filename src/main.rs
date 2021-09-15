use std::io;

fn main() {
    let words = get_input();
    let n:u32 = words[0].parse().unwrap();
    let a:u32 = words[1].parse().unwrap();
    let b:u32 = words[2].parse().unwrap();
    let mut ans = 0;
    for i in 1..n+1 {
        let sum = i.to_string().chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum();
        if a <= sum && sum <= b {
            ans += i;
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


