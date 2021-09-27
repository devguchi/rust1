fn main() {
    print!("{} ", fact(20));
}

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


