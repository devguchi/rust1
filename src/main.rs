use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input!{
        n: usize,
        a:[(u64, [Usize1]); n]
    }
    let mut ok_list: [bool; 1000000] = [false; 1000000];
    let result = study(n-1, &a, &mut ok_list);
    println!("{}", result);
}

fn study(no: usize, a:&Vec<(u64, Vec<usize>)>, mut ok_list: &mut [bool; 1000000]) -> u64 {
    if ok_list[no] { return 0 }
    ok_list[no] = true;
    let mut time = a[no].0;
    if a[no].1.len() < 1 { return time }
    for next_no in a[no].1.iter() {
        time += study(*next_no, &a, &mut ok_list);
    }
    time
}
