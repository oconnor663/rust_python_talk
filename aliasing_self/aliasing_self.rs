struct Person {
    favorite_foods: Vec<&'static str>,
}

impl Person {
    fn add_favorites(&mut self, other: &Person) {
        for food in &other.favorite_foods {
            self.favorite_foods.push(food);
        }
    }
}

fn main() {
    let mut alice = Person {
        favorite_foods: vec!["apples"],
    };
    alice.add_favorites(&alice);
}
