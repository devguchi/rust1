use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let mut handles = Vec::new();
    let hoge = Arc::new(Mutex::new(vec![1; 10]));
    let hoge2 = Arc::new(Mutex::new(0));
    for x in 0..10 {
        let data_ref = hoge.clone();
        let data2_ref = hoge2.clone();
        let handle = thread::spawn(move ||{
            let mut data = data_ref.lock().unwrap();
            let mut data2 = data2_ref.lock().unwrap();
            data[x] = x;
            *data2 += x;
            println!("thread {} {} {}", x, data[x], data2);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    dbg!(hoge);
    dbg!(hoge2);
}




