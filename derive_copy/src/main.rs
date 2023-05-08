#![allow(dead_code)]

#[derive(Copy, Clone)]
enum Foo {
    X,
}

fn main() {
    println!("Hello, world!");
}

#[derive(Copy, Clone)]
enum WebEvent<'a> {
    PageLoad,
    PageUnload,
    KeyPress(char),
    // Paste(String),
    //       ------ this field does not implement `Copy`
    Paste(&'a str),
    Click { x: i64, y: i64 },
}
