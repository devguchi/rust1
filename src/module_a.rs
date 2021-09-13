mod module_a_1;
use crate::module_b;

pub fn hoge() {
    println!("module_a");
    module_a_1::hoge();
    module_b::hoge();
}
