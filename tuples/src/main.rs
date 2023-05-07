// A tuple struct has no names:
#[derive(Debug)]
struct SimpleMushroom(String, i32);

fn main() {
    // Creating a tuple struct:
    let simple = SimpleMushroom("Oyster".to_string(), 7);
    println!("{:#?}", simple);
    println!("{}, {}", simple.0, simple.1);
    println!("{:#?}", ("foo", 100, 'x'));
}
