pub struct NumberList {
    numbers: Vec<i32>,
}

impl NumberList {
    pub fn take_all(&mut self, other: &mut NumberList) {
        self.numbers.extend(&other.numbers);
        other.numbers.clear();
    }
}

fn main() {
    let mut list_a = NumberList {
        numbers: vec![1, 2, 3],
    };
    let mut list_b = NumberList {
        numbers: vec![4, 5, 6],
    };
    list_a.take_all(&mut list_b);
    assert_eq!(list_a.numbers, [1, 2, 3, 4, 5, 6]);
    assert_eq!(list_b.numbers, []);

    // OOPS: What does this do?
    list_a.take_all(&mut list_a);
    assert_eq!(list_a.numbers, [1, 2, 3, 4, 5, 6]);
}
