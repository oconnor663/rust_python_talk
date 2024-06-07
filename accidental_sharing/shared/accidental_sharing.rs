pub struct Person<'a> {
    pub favorite_foods: &'a Vec<String>,
}

fn main() {
    let default_foods = vec!["donuts".into()];

    let alice = Person {
        favorite_foods: &default_foods,
    };
    alice.favorite_foods.push("apples".into());

    let bob = Person {
        favorite_foods: &default_foods,
    };
    bob.favorite_foods.push("bananas".into());

    println!("alice: {:?}", alice.favorite_foods);
    println!("bob: {:?}", bob.favorite_foods);
}
