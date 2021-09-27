fn main() {
    let mut fact: [i64; 11] = [1; 11];
    for i in 0..11 {
        if i > 0 {
            fact[i] = fact[i-1]*(i as i64);
        }
        println!("{}! = {}", i, fact[i]);
    }
}

