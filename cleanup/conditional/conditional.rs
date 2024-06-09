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

fn random_mess(mess1: Mess, mess2: Mess) -> Mess {
    if rand::random() {
        mess1
    } else {
        mess2
    }
}

fn main() {
    let mess1 = Mess::new("first".into());
    let mess2 = Mess::new("second".into());
    let mess = random_mess();
    mess.look()
}
