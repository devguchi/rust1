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

    let result: Result<i32, String> = Ok(200);
    let _result2 = result.and_then(func);
    let result3: Result<i32, String> = Err("error".to_string());
    let _result4 = result3.and_then(func);

    let vec1 = vec![1,2,3,4,5];
    for el in &vec1 {
        println!("{}", el);
    }
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

fn func(code: i32) -> Result<i32, String> {
    println!("code: {}", code);
    Ok(100)
}

