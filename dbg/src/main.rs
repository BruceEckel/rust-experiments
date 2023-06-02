#![allow(dead_code)]

#[derive(Debug, Clone)]
struct Variety<'a> {
    d: u32,
    s: &'a str,
}

fn main() {
    let v = &Variety {
        d: 10,
        s: "howdy",
    };
    let q = &["foo", "bar", "baz"];
    dbg!();
    dbg!(v,);
    dbg!("debugging label:", v, q);
}
