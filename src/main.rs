use proconio::input;

fn main() {
    input! {
        r: f32,
    }
    println!("{}", r*2.0*std::f32::consts::PI)
}

