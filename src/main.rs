use proconio::input;
 
fn main() {
    input!{
        n: usize,
        x: usize,
        a: [usize; n]
    }
    let mut know_list = vec![false; n];
    teach(x-1, &mut know_list, &a);
    let count = (0..n).filter(|&i| know_list[i]).count();
    println!("{}", count);
}
 
fn teach(i:usize, mut know_list: &mut Vec<bool>, a: &Vec<usize>) {
    know_list[i] = true;
    if know_list[a[i]-1] == true {return;}
    teach(a[i]-1, &mut know_list, &a);
}