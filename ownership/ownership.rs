use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn background_work(mut v: Vec<i32>) -> Vec<i32> {
    sleep(Duration::from_millis(100));
    v.push(42);
    v
}

fn main() {
    let mut v = Vec::new();
    assert_eq!(v.len(), 0);
    let thread = thread::spawn(|| background_work(v));
    // OOPS: swap these two lines
    v = thread.join().unwrap();
    assert_eq!(v.len(), 1);
}
