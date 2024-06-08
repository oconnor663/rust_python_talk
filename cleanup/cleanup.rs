struct Mess {
    size: String,
}

impl Mess {
    fn new(size: String) -> Mess {
        println!("Make a {} mess.", size);
        Mess { size }
    }

    fn look(&self) {
        println!("Look at this {} mess!", self.size);
    }
}

impl Drop for Mess {
    fn drop(&mut self) {
        println!("Clean up the {} mess.", self.size);
    }
}

fn main() {
    let huge_mess = Mess::new("huge".into());
    huge_mess.look();
}
