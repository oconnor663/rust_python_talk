use std::collections::HashSet;

pub struct Person {
    favorite_foods: HashSet<String>,
}

impl Person {
    pub fn add_food(&mut self, food: &str) {
        self.favorite_foods.insert(String::from(food));
    }
}

pub fn set(strings: &[&str]) -> HashSet<String> {
    strings.into_iter().copied().map(String::from).collect()
}

#[test]
fn test_foods() {
    let default_foods = set(&["donut"]);
    let mut alice = Person {
        favorite_foods: default_foods,
    };
    alice.add_food("apple");
    // OOPS: Alice and Bob are both referencing the same set.
    let mut bob = Person {
        favorite_foods: default_foods,
    };
    bob.add_food("banana");
    assert_eq!(alice.favorite_foods, set(&["donut", "apple"]));
    assert_eq!(bob.favorite_foods, set(&["donut", "banana"]));
}
