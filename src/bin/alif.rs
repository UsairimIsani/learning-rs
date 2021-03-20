trait Walk {
    fn walkonfoot(&self);
}
struct Human {
    name: String,
}

struct Animal;

impl Animal {
    fn new(&self) -> Self {
        Animal
    }
}

impl Human {
    // constructor -> associated fn -> no self i.e not init alread
    fn new(name: String) -> Self {
        Human { name }
    }
}

impl Walk for Animal {
    fn walkonfoot(&self) {
        println!("I always walk.. you idiot")
    }
}

impl Walk for Human {
    fn walkonfoot(&self) {
        println!("OHK... I'll ")
    }
}

fn main() {
    let usairim = Human::new("USAIRIM_CHINGAARI".to_string());
    usairim.walkonfoot();
}
