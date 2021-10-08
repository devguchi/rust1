use std::io;

fn main() {
    let weather:Vec<char> = get_input()[0].chars().collect();
    let mut max = 0;
    let mut days = 0;
    for w in weather.iter() {
        if *w == 'R' {
            days += 1;
            if max < days { max = days.clone(); }
        } else {
            days = 0;
        }
    }
    println!("{}", max);
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

