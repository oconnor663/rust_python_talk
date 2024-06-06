pub struct Person<'a> {
    pub favorite_numbers: &'a Vec<i32>,
}

fn main() {
    let default_numbers = vec![42];
    let mut william = Person {
        favorite_numbers: &default_numbers,
    };
    william.favorite_numbers.push(1066);
    let mut george = Person {
        favorite_numbers: &default_numbers,
    };
    george.favorite_numbers.push(1776);
    println!("william: {:?}", william.favorite_numbers);
    println!("george: {:?}", george.favorite_numbers);
}
