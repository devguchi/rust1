use num::integer::gcd;

fn main() {
    let v = vec![24, 48];
    let ans = v[1..].iter().fold(v[0], |ans, &x| gcd(ans, x));
    println!("{}", ans);
}
