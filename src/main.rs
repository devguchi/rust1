use std::io;

fn main() {
    let n = get_input_i64()[0] as usize;
    let h = get_input_i64();
    let mut score:Vec<i64> = vec![0];
    for i in 1..n {
        let idx = i as usize;
        if i == 1 {
            score.push((h[idx]-h[idx-1]).abs());
        } else {
            let s1 = (h[idx]-h[idx-1]).abs()+score[idx-1];
            let s2 = (h[idx]-h[idx-2]).abs()+score[idx-2];
            if s1 < s2 {
                score.push(s1);
            } else {
                score.push(s2);
            }
        }
    }
    println!("{}", score[n-1]);
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

