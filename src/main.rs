use std::io;

fn main() {
    let nk = get_input_i64();
    let n = nk[0];
    let k = nk[1];
    let mut total = 0;
    for i in 1..n+1 {
        for j in 1..k+1 {
            total += 100*i+j;
        }
    }
    println!("{}", total);
}

fn get_input_i64() -> Vec<i64> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.parse().unwrap()).collect()
}

