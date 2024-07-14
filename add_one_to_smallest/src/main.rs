use std::sync::{Mutex, MutexGuard};

fn add_one_to_smallest(ints: &[Mutex<i32>]) {
    let mut smallest: &mut i32 = &mut *ints[0].lock().unwrap();
    for i in 1..ints.len() {
        let next: &mut i32 = &mut *ints[i].lock().unwrap();
        if *next < *smallest {
            smallest = next;
        }
    }
    *smallest += 1;
}

static INTS: [Mutex<i32>; 5] = [
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
    Mutex::new(0),
];

fn main() {
    for _ in 0..3 {
        add_one_to_smallest(&INTS);
    }
    dbg!(&INTS);
}
