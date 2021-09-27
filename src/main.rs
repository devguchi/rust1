mod exectime;

fn main() {
    let start = exectime::start();
    for i in 0..40 {
        print!("{} ", fib(i));
    }
    exectime::end(start);
}

fn fib(n:i64) -> i64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
}

