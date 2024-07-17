#![allow(unused)]

use std::sync::{Mutex, MutexGuard};

fn add_one_to_smallest(ints: &[Mutex<i32>]) {
    let mut smallest_guard: MutexGuard<i32> = ints[0].lock().unwrap();
    for i in 1..ints.len() {
        let next_guard: MutexGuard<i32> = ints[i].lock().unwrap();
        if *next_guard < *smallest_guard {
            smallest_guard = next_guard;
        }
    }
    *smallest_guard += 1;
}

#[test]
fn test_sync() {
    let ints = [
        Mutex::new(0),
        Mutex::new(0),
        Mutex::new(0),
        Mutex::new(0),
        Mutex::new(0),
    ];
    for _ in 0..3 {
        add_one_to_smallest(&ints);
    }
    assert_eq!(*ints[2].lock().unwrap(), 1);
}

mod regular {
    fn add_one_to_smallest(ints: &mut [i32]) {
        let mut smallest_index = 0;
        for next_index in 1..ints.len() {
            if ints[next_index] < ints[smallest_index] {
                smallest_index = next_index;
            }
        }
        ints[smallest_index] += 1;
    }

    #[test]
    fn test_add() {
        let mut ints = [0, 0, 0, 0, 0];
        for _ in 0..3 {
            add_one_to_smallest(&mut ints);
        }
        assert_eq!(ints, [1, 1, 1, 0, 0]);
    }
}
