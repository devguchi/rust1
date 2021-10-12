use proconio::input;

fn main() {
    input! {
        s: String,
        mut t: String,
    }
    let mut ans = "No"; 
    t.pop();
    if s == t {
        ans = "Yes";
    }
    println!("{}", ans);
}
