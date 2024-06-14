struct Person<'food> {
    favorite_foods: &'food Vec<&'static str>,
}

fn main() {
    let donuts = vec!["donuts"];

    let alice = Person {
        favorite_foods: &donuts,
    };
    // error: cannot borrow `*alice.favorite_foods` as mutable
    alice.favorite_foods.push("apples");

    let bob = Person {
        favorite_foods: &donuts,
    };
    // error: cannot borrow `*bob.favorite_foods` as mutable
    bob.favorite_foods.push("bananas");

    println!("{:?}", alice.favorite_foods);
    println!("{:?}", bob.favorite_foods);
}
