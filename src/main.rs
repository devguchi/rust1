use proconio::*;

#[fastout]
fn main() {
    input!{ s: String, k: u32 }
    let c_list:Vec<char> = s.chars().collect();
    let mut ans = 0;
    let mut right = 1;
    let n = s.len();
    for i in 0..n {
        loop {
            let mut cnt:u32 = 0;
            for c in &c_list[i..right] {
                if *c == '.' {cnt += 1;}
            }
            if cnt <= k {
                if right - i > ans {ans = right - i;}
                right+=1;
                if right > n {right = n; break;}
                println!("n:{} i:{} right:{} cnt:{} ans:{}", n,i,right,cnt,ans);
            } else {
                break;
            }
        }
    }
    println!("{}", ans)
}
