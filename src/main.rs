fn main() {
    println!("Hello, world!");
    println!("{}", calc(12, 23));
    println!("{}", hello("hoge"));
    println!("{}", hoge(12, 23));

    let ossan = Ossan {
        name: "SIMURA KEN".to_string(),
        age: 80
    };

    println!("{} {}", ossan.name, ossan.age);
}

fn calc(x: i64, y:i64) -> i64 {
    x + y
}

fn hello(s: &str) -> String {
    println!("{}", s);
    let s2: String = s.to_string();
    println!("{}", s2);
    s2+"HOGE"
}

fn hoge(x: i32, y: i32) -> i32 {
    let t = (x, y);
    let a: [i32; 3] = [31, t.0, t.1];
    a[0] + a[1] + a[2]
}

struct Ossan {
    name: String,
    age: u32
}

