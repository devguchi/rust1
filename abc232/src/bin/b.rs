use proconio::*;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input!{ s: Chars, t: Chars }
    let az:Vec<char> = vec![
        'a', 'b', 'c', 'd', 'e', 
        'f', 'g', 'h', 'i', 'j', 
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 
        'u', 'v', 'w', 'x', 'y', 
        'z',
    ];
    let mut idx = 0;
    let mut ans = "Yes";
    for i in 0..t.len() {
        let mut ti:usize = az.iter().position(|&e| e == t[i]).unwrap();
        let si:usize = az.iter().position(|&e| e == s[i]).unwrap();
        if ti < si { ti += 26; }
        if i == 0 {
            idx = ti - si;
        } else {
            let _idx = ti - si;
            if idx != _idx {
                ans = "No";
                break;
            }
        }
    }
    println!("{}", ans);
}
