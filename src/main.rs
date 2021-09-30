use std::io;
use std::cmp::min;


fn main() {
    let n = get_input_i64()[0] as usize;
    let mut dp:[i64; 100010] = [std::i64::MAX; 100010];
    dp[0] = 0;
    for i in 1..n+1 {
        let iu = i as usize;
        let mut pow6 = 1;
        let mut pow9 = 1;
        while pow6 <= i {
            dp[iu] = min(dp[iu], dp[iu-pow6]+1);
            pow6 *= 6;
        }
        while pow9 <= i {
            dp[iu] = min(dp[iu], dp[iu-pow9]+1);
            pow9 *= 9;
        }
    }
    println!("{}", dp[n]);
}

fn get_input_i64() -> Vec<i64> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.parse().unwrap()).collect()
}

