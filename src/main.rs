use proconio::input;

fn main() {
    input! {
        k: i64,
        ab: [i64; 2]
    }
    for i in ab[0]..ab[1]+1 {
        if i%k == 0 {
            println!("OK");
            std::process::exit(0);
        }
    }
    println!("NG");
}
