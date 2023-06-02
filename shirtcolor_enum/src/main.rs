#[derive(Debug)]
enum ShirtColor {
    Red,
    Blue
}

fn main() {
    let _shirts = vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue];
    println!("{:?}", _shirts);
}
