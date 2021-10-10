use std::io;
use std::collections::HashSet;

// Vec<String> -> HashSet<String>
fn vec_string_hashset(words:&Vec<String>) -> HashSet<String> {
    words.clone().into_iter().collect()
}

// Vec<String> -> HashSet<char>
fn string_hashset_char(s:&String) -> HashSet<char> {
    let vec:Vec<char> = s.clone().chars().collect();
    vec.into_iter().collect()
}

// 各桁の合計
fn digits_sum(s:&String) -> u32 {
    s.chars().map(|c| c.to_digit(10).unwrap()).sum()
}

// 各桁をu32のvecにする
fn string_vec_u32(s:&String) -> Vec<u32> {
    s.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

// a-zのcharのvec
fn a_z() -> Vec<char> {
    (b'a'..=b'z').map(|b| b as char).collect()
}

// 階乗（再帰）
fn fact(n:i64) -> i64 {
    if n < 1 {
        1
    } else {
        fact(n-1) * n
    }
}

// 階乗（DP）
fn fact(n:usize) -> i64 {
    let mut memo: [i64; 1000] = [1; 1000];
    _fact(n, &mut memo)
}

fn _fact(n:usize, memo: &mut [i64; 1000]) -> i64 {
    if n < 2 || memo[n] > 1 {
        memo[n]
    } else {
        memo[n] = _fact(n-1, memo) * (n as i64);
        memo[n]
    }
}

// 型名の表示
fn typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

// 二分探索
// 探している値の一番小さいindexを返す
fn lower_bound(v:&Vec<i64>, s:&i64) -> Result<usize, usize> {
    let mut low = 0;
    let mut high = v.len();
    let mut exist = false;
    while low != high {
        let mid = (low + high)/2;
        if v[mid] < *s {
            low = mid+1;
        } else if v[mid] > *s {
            high = mid;
        } else { 
            high = mid;
            exist = true;
        }
    }
    if exist { Ok(low) } else { Err(low) }
}

// 探している値の一番大きいindexを返す
fn upper_bound(v:&Vec<i64>, s:&i64) -> Result<usize, usize> {
    let mut low = 0;
    let mut high = v.len();
    let mut exist = false;
    while low != high {
        let mid =  (low+high)/2;
        if v[mid] > *s {
            high = mid;
        } else if v[mid] < *s {
            low = mid+1;
        } else { 
            low = mid+1;
            exist = true;
        }
    }
    if exist { Ok(low-1) } else { Err(low) }
}