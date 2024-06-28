use std::sync::Mutex;

static X: Mutex<i32> = Mutex::new(0);

fn add_500k() {
    for _ in 0..500_000 {
        *X.lock().unwrap() += 1;
    }
}

fn main() {
    let thread1 = std::thread::spawn(add_500k);
    let thread2 = std::thread::spawn(add_500k);
    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("{}", X.lock().unwrap());
}
