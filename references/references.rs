pub struct Person {
    pub favorite_numbers: Vec<i32>,
}

fn main() {
    let default_numbers = vec![42];

    let mut william = Person {
        favorite_numbers: default_numbers.clone(),
    };
    william.favorite_numbers.push(1066);

    let mut george = Person {
        favorite_numbers: default_numbers,
    };
    george.favorite_numbers.push(1776);

    println!("william: {:?}", william.favorite_numbers);
    println!("george: {:?}", george.favorite_numbers);
}
