use std::time::Instant;

pub fn start() -> Instant {
    Instant::now()
}

pub fn end(start: Instant) {
    let end = start.elapsed();
    let sec = end.as_secs();
    let subsec = end.subsec_nanos() / 1_000_000;
    println!("\ntime: {}.{:09}", sec, subsec);
}

