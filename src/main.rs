fn main() {
    let h = Hoge {
        name: "taro".to_string()
    };
    println!("1");
    println!("{}", h.name);
    println!("2");
    let h2 = Hoge {
        name: "jiro".to_string()
    };
    println!("{}", h2.name);
    println!("3");
}

struct Hoge {
    name: String
}

impl Drop for Hoge {
    fn drop(&mut self) {
        println!("drop {}", self.name);
    }
}



