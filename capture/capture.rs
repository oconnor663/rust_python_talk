fn main() {
    let mut functions = Vec::new();

    for i in 0..10 {
        // error: `i` does not live long enough
        functions.push(|| println!("{i}"));
    }

    for f in functions {
        f();
    }
}
