struct Mess {
    size: String,
}

impl Mess {
    fn new(size: String) -> Mess {
        println!("Make a {} mess.", size);
        Mess { size }
    }

    fn look(&self) {
        println!("Look at the {} mess!", self.size);
    }
}

impl Drop for Mess {
    fn drop(&mut self) {
        println!("Clean up the {} mess.", self.size);
    }
}

fn main() {
    let giant_mess = Mess::new("giant".into());
    giant_mess.look();
}
