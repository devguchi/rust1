use std:: env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num1 = args[1].parse::<u32>().unwrap();
    let num2 = args[2].parse::<u32>().unwrap();
    println!("num1 + num2 = {}", num1 + num2);
}

