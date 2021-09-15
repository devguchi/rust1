use std::io;

fn main() {
    let a_max:u32 = get_input()[0].parse().unwrap();
    let b_max:u32 = get_input()[0].parse().unwrap();
    let c_max:u32 = get_input()[0].parse().unwrap();
    let mut count = 0;
    let x:u32 = get_input()[0].parse().unwrap();
    for a in 0..a_max+1 {
        if x < 500*a {
            break;
        }
        for b in 0..b_max+1 {
            if x < 500*a+100*b {
                break;
            }
            let c = (x-500*a-100*b)/50;
            if c <= c_max {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

