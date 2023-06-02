trait Animal {
    fn snuggle(&self);
    fn eat(&self);
}

trait Cat: Animal {
    fn meow(&self);
}

trait Dog: Animal {
    fn bark(&self);
}

struct Bowser;

impl Animal for Bowser {
    fn snuggle(&self) {
        println!("Bowser snuggling")
    }
    fn eat(&self) {
        println!("Bowser eating")
    }
}

impl Dog for Bowser {
    fn bark(&self) {
        println!("Bowser barking")
    }
}

fn main() {
    let b: Box<dyn Dog> = Box<Bowser>();
    // b.snuggle();
    // b.eat();
    b.bark();
}
