use proconio::*;

#[fastout]
fn main() {
    input!{ s: String, k: u32 }
    let c_list:Vec<char> = s.chars().collect();
    let n = c_list.len();
    let mut ans = 0;
    let mut right = 1;
    let mut dot_cnt:u32 = 0;
    for i in 0..n {
        if i > 0 && c_list[i-1] == '.' {dot_cnt -= 1;}
        loop {
            if c_list[right-1] == '.' {dot_cnt += 1;}
            if dot_cnt <= k {
                if right - i > ans {ans = right - i;}
                right+=1;
                if right > n {right = n; break;}
            } else {
                dot_cnt -= 1;
                break;
            }
        }
    }
    println!("{}", ans)
}
