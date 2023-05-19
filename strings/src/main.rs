// format!() seems more expedient than String::from()

// The only use case for Box<str> over String is optimising for
// memory usage when the string is fixed and you do not intend
// to append or remove from it.

fn main() {
    let w = "world!";
    let hw = Box::new("Hello, ".to_owned() + w);  // Can't do any formatting with {}
    println!("{hw}");
    let gb = "Goodbye ";
    let gw = String::from("Alas, ".to_owned() + gb + w);  // Can't do any formatting with {}
    println!("{gw}");
    let f = format!("{hw}, ...Goodbye... {}.", gw); // Can do all the formatting you want...
    println!("{f}");

    let mut sn = String::new();
    sn.push('X');
    sn.push_str("foo");
    sn.push_str("bar");
    println!("{sn}");

    // https://www.poetryfoundation.org/poems/46565/ozymandias
    for w in "Look on my Works, ye Mighty, and despair!".split_whitespace() {
        print!("{w} ");
    }
}
