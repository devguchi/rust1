use std::io;

fn main() {
    let n = get_input_i64()[0] as usize;
    let a_list = get_input_lines(n);
    let mut ok_list: [bool; 1000000] = [false; 1000000];
    let result = study(n, &a_list, &mut ok_list);
    println!("{}", result);
}

fn study(no: usize, a_list:&Vec<Vec<i64>>, mut ok_list: &mut [bool; 1000000]) -> i64 {
    if ok_list[no-1] { return 0 }
    ok_list[no-1] = true;
    let mut time = a_list[no-1][0];
    if a_list[no-1].len() < 3 { return time }
    for i in 0..a_list[no-1][1] {
        let next_no = a_list[no-1][i as usize + 2] as usize;
        time += study(next_no, &a_list, &mut ok_list);
    }
    time
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

fn get_input_lines(line_len:usize) -> Vec<Vec<i64>> {
    let mut vec:Vec<Vec<i64>> = vec![];
    let mut input:Vec<i64>;
    for _ in 0..line_len {
        input = get_input_i64();
        vec.push(input);
    }
    vec
}

