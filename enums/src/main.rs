#![allow(dead_code)]

#[derive(Copy, Clone)]
enum WebEvent<'a> {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(&'a str),
    Click { x: i64, y: i64 },
}

use crate::WebEvent::*; // Don't have to preface everything with WebEvent::

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        PageLoad => println!("page loaded"),
        PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        KeyPress(c) => println!("pressed '{}'.", c),
        Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

// Must impl all of WebEvent. That is, you can't cherry pick
// and implement one variant by itself, like Click or Paste.
impl WebEvent<'_> {
    fn respond(&self) {
        match self {
            PageLoad => {}
            PageUnload => {}
            KeyPress(_c) => {}
            Paste(_s) => {}
            Click { x: _, y: _ } => {}
        }
    }
}

fn main() {
    let pressed = KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = Paste("my text");
    let click = Click { x: 20, y: 80 };
    let load = PageLoad;
    let unload = PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    pressed.respond();
    pasted.respond();
    click.respond();
    load.respond();
    unload.respond();
}
