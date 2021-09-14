use std:: env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args[1].to_string();
    println!("file: {}", file_name);
    let f = File::open(file_name);
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("ERROR! {:?}", error);
        },
    };
    dbg!(f);
}


