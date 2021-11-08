use proconio::input;
// use proconio::marker::Usize1;

fn main() {
    input!{
        s: String,
    }
    let v:Vec<char> = s.chars().collect();
    let mut result = "No";
    if v[2] == v[3] && v[4] == v[5] {
        result = "Yes";
    }
    println!("{}", result);
}
