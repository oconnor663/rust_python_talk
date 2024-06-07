pub struct Person<'a> {
    pub favorite_foods: &'a mut Vec<String>,
}

fn main() {
    let mut default_foods = vec!["donuts".into()];

    let alice = Person {
        favorite_foods: &mut default_foods,
    };
    alice.favorite_foods.push("apples".into());

    let bob = Person {
        // OOPS: We can't take two mutable references to the same list.
        favorite_foods: &mut default_foods,
    };
    bob.favorite_foods.push("bananas".into());

    println!("alice: {:?}", alice.favorite_foods);
    println!("bob: {:?}", bob.favorite_foods);
}
