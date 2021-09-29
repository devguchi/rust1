use std::io;

fn main() {
    let n = get_input_i64()[0] as usize;
    let mut h:Vec<Vec<i64>> = vec![];
    for _ in 0..n {
        h.push(get_input_i64());
    }
    let mut dp:[[i64;3]; 100010] = [[0,0,0]; 100010];
    for i in 0..n {
        for j in 0..3 {
            if i == 0 {
                dp[i][j] = h[i][j];
            } else {
                for k in 0..3 {
                    if j == k {continue;}
                    dp[i][j] = cmax(dp[i][j], dp[i-1][k]+h[i][j]);
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..3 {
        ans = cmax(ans, dp[n-1][i]);
    }
    println!("{}", ans);
}

fn get_input_i64() -> Vec<i64> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.parse().unwrap()).collect()
}

fn cmax(a:i64, b:i64) -> i64 {
    if a > b { a } else { b }
}
