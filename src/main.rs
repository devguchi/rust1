use std::thread;

fn main() {
    let mut a = 100;
    thread::spawn(move ||{
        println!("hoge");
        for x in 0..100 {
            println!("hoge {} {}", x, a);
        }
    });
    a = 200;
    let handle = thread::spawn(move ||{
        println!("hoge2");
        for x in 0..10 {
            a += x;
            println!("hoge2 {} {}", x, a);
        }
    });
    let _ = handle.join();
    println!("hoge3 {}", a);
}




