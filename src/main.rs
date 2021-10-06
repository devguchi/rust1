use std::io;

fn main() {
    let dts = get_input_i64();
    let d = dts[0] as f64;
    let t = dts[1] as f64;
    let s = dts[2] as f64;
    if d/s <= t {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn get_input_i64() -> Vec<i64> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.parse().unwrap()).collect()
}
