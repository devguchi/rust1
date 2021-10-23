use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[f64;2]; n]
    }
    let mut cnt = 0;
    for i in 0..n {
        for j in 0..n {
            for h in 0..n {
                if i == j || i == h || j == h {continue;}
                let ij;
                let ih;
                let jh;
                if a[i][0] == a[j][0] {
                     ij = -1.0;
                } else {
                    ij = ((a[i][1]-a[j][1])/(a[i][0]-a[j][0])).abs();
                }
                if a[i][0] == a[h][0] { 
                    ih = -1.0; 
                } else {
                    ih = ((a[i][1]-a[h][1])/(a[i][0]-a[h][0])).abs();
                }
                if a[j][0] == a[h][0] { 
                    jh = -1.0; 
                } else {
                    jh = ((a[j][1]-a[h][1])/(a[j][0]-a[h][0])).abs();
                }
                if ij != ih || ij != jh || ih != jh {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt/6);
}

