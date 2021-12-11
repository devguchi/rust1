use proconio::*;

#[fastout]
fn main() {
    input!{
        n: usize, q:usize, 
        mut a: [usize; n], 
        x_list: [usize; q]
    }
    a.sort();
    for x in x_list {
        let i = lower_bound(&a, &x);
        println!("{}", n-i);
    }
}

fn lower_bound(v:&Vec<usize>, s:&usize) -> usize {
    let mut low = 0;
    let mut high = v.len();
    while low != high {
        let mid = (low + high)/2;
        if v[mid] < *s {
            low = mid+1;
        } else { 
            high = mid;
        }
    }
    low
}
