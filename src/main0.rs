use std::io;

fn main() {
    let nk = get_input_i64();
    let n = nk[0];
    let k = nk[1];
    let h = get_input_i64();
    const INF:i64 = std::i64::MAX;
    let mut score:Vec<i64> = vec![0];
    for i in 1..n {
        let iu = i as usize;
        let mut s = INF;
        for j in 1..k+1 {
            if i-j < 0 {
                break;
            }
            let ju = j as usize;
            s = cmin(s, (h[iu]-h[iu-ju]).abs()+score[iu-ju]);
        }
        score.push(s);
    }
    println!("{}", score[(n as usize)-1]);
}

fn cmin(a:i64, b:i64) -> i64 {
    if a > b { b } else { a }
}

fn get_input_i64() -> Vec<i64> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.parse().unwrap()).collect()
}

