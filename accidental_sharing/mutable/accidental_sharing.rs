struct Person<'food> {
    favorite_foods: &'food mut Vec<&'static str>,
}

fn main() {
    let mut donuts = vec!["donuts"];
    let alice = Person {
        // first mutable borrow occurs here
        favorite_foods: &mut donuts,
    };
    alice.favorite_foods.push("apples");
    let bob = Person {
        // error: cannot borrow `donuts` as mutable more than once at a time
        favorite_foods: &mut donuts,
    };
    bob.favorite_foods.push("bananas");

    println!("{:?}", alice.favorite_foods);
    println!("{:?}", bob.favorite_foods);
}
