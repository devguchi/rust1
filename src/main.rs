use std::io;
use std::cmp::min;


fn main() {
    let n = get_input_i64()[0];
    let mut memo: [i64; 100010] = [-1; 100010];
    println!("{}", rec(n, &mut memo));
}

fn rec(n:i64, memo: &mut [i64; 100010]) -> i64 {
    let nu = n as usize;
    if n == 0 {return 0;}
    if memo[nu] != -1 {return memo[nu];}
    let mut res = n.clone();
    let mut pow6 = 6;
    let mut pow9 = 9;
    while pow6 <= n {
        res = min(res, rec(n-pow6, memo)+1);
        pow6 *= 6;
    }
    while pow9 <= n {
        res = min(res, rec(n-pow9, memo)+1);
        pow9 *= 9;
    }
    memo[nu] = res;
    res
}

fn get_input_i64() -> Vec<i64> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.parse().unwrap()).collect()
}

