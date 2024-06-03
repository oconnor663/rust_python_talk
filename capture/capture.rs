fn main() {
    let mut functions = Vec::new();
    for i in 0..10 {
        functions.push(|| i + 1);
    }
    let results: Vec<i32> = functions.iter().map(|f| f()).collect();
    println!("{:?}", results);
}
