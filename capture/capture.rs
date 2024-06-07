fn main() {
    let mut functions = Vec::new();

    for i in 0..10 {
        // OOPS: This captures i by reference.
        functions.push(|| println!("{i}"));
    }

    for f in functions {
        f();
    }
}
