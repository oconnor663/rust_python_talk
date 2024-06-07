pub struct Person {
    pub favorite_foods: Vec<String>,
}

fn main() {
    let default_foods = vec!["donuts".into()];

    let mut alice = Person {
        favorite_foods: default_foods,
    };
    alice.favorite_foods.push("apples".into());

    let mut bob = Person {
        favorite_foods: default_foods,
    };
    bob.favorite_foods.push("bananas".into());

    println!("alice: {:?}", alice.favorite_foods);
    println!("bob: {:?}", bob.favorite_foods);
}
