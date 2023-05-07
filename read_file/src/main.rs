use std::fs;

fn main() {
    let foo = fs::read_to_string("main.rs").unwrap();
    println!("{}", foo);
}
