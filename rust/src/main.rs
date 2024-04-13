fn fib(x: u64) -> u64 {
    if x <= 1 {
        1
    } else {
        fib(x - 1) + fib(x - 2)
    }
}

fn main() {
    let x: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    println!("{}", fib(x));
}
