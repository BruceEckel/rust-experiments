#[allow(dead_code)]
#[derive(Debug, Default)]
struct Foo {
    i: i32,
    f: f64,
}

impl Foo {
    fn f1(&self) {}
}

impl Foo {
    fn f2(&self) {}
}

fn main() {
    let foo: Foo = Default::default();
    println!("Hello, world! {:?}", foo);
    foo.f1();
    foo.f2();
}
