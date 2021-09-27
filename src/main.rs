use std::io;

fn main() {
    let mut p = get_input_i64()[0];
    let mut total_num = 0;
    for i in 0..11 {
        let unit_price = fact(10-i);
        for num in 0..100 {
            if unit_price * num > p {
                total_num += num-1;
                p -= unit_price * (num-1);
                break;
            } 
        }
    }
    println!("{}", total_num);
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

fn fact(n:i64) -> i64 {
    if n < 1 {
        1
    } else {
        fact(n-1) * n
    }
}
