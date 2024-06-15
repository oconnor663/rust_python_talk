struct Person {
    favorite_foods: Vec<&'static str>,
}

fn main() {
    let donuts = vec!["donuts"];

    let mut alice = Person {
        favorite_foods: donuts, // value moved here
    };
    alice.favorite_foods.push("apples");

    let mut bob = Person {
        favorite_foods: donuts, // error: use of moved value: `donuts`
    };
    bob.favorite_foods.push("bananas");

    println!("{:?}", alice.favorite_foods);
    println!("{:?}", bob.favorite_foods);
}
