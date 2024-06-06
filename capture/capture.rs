fn main() {
    let mut functions = Vec::new();

    for i in 0..10 {
        functions.push(move || println!("{i}"));
    }

    for f in functions {
        f();
    }
}
