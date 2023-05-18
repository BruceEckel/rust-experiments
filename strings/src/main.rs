// The only use case for Box<str> over String is optimising for
// memory usage when the string is fixed and you do not intend
// to append or remove from it.

// format!() seems more expedient than String::from()

fn main() {
    let w = "world!";
    let b = Box::new("Hello, ".to_owned() + w);  // Can't do any formatting with {}
    println!("{b}");
    let g = String::from("Goodbye, ".to_owned() + w);  // Can't do any formatting with {}
    println!("{g}");
    let f = format!("{} ...Goodbye...", g); // Can do all the formatting you want...
    println!("{f}");
}
