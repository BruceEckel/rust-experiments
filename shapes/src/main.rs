// Polymorphism: dyn vs. impl
use std::ops::Deref;

trait Shape {
    fn draw(&self);
}

struct Circle {
    radius: u32,
}

impl Shape for Circle {
    fn draw(&self) {
        // Self knows it is a Circle:
        println!("Circle({})", self.radius);
    }
}

struct Rectangle {
    length: u32,
    width: u32,
}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Square({}, {})", self.length, self.width);
    }
}

// Like C++ templates--new version generated for each argument type:
fn show_impl(s: impl Shape) {
    s.draw();
}
// Invisible automatically-generated function overloading!

// `&` because size is unknown, `dyn` because type is unknown.
// `dyn` produces a "fat pointer" including a vtable.
fn show_dyn(s: &dyn Shape) {
    s.draw();
}
// Shape isn't automatically burdened with a vtable! ("zero-cost abstractions")

fn main() {
    const SHAPES: [&dyn Shape; 2] = [
        &Circle { radius: 1 },
        &Rectangle { length: 2, width: 3, },
    ];
    SHAPES[0].draw();
    SHAPES[1].draw();

    let (c, r) = (
        Circle { radius: 4 },
        Rectangle { length: 5, width: 6, },
    );
    show_impl(c); // Creates a version of show_impl() for Circle
    show_impl(r); // Creates a version of show_impl() for Square
    // show_impl(*SHAPES[0]);  // Doesn't know the type

    for shape in SHAPES {
        show_dyn(shape);
        // Doesn't know the type at compile time:
        // show_impl(shape);
    }
    vec_of_shapes();
    impl_variations();
}

fn vec_of_shapes() {
    // Rust can't infer the type:
    let shapes: Vec<&dyn Shape> = vec![
        &Circle { radius: 11 },
        &Rectangle { length: 12, width: 13 },
        &Circle { radius: 14 },
        &Rectangle { length: 15, width: 16 },
    ];
    for shape in shapes.iter() {
        show_dyn(*shape);
    }
}

fn impl_variations() {  // Thanks to @mp
    impl Shape for Box<dyn Shape> {
        fn draw(&self) {
            print!("Box<dyn Shape> draw: ");
            self.deref().draw()
        }
    }

    // This isn't matching for some reason. @mp didn't know, but
    // said "never seen a &dyn Trait in practice — only Box<dyn Trait>"
    impl Shape for &dyn Shape {
        fn draw(&self) {
            print!("&dyn Shape draw: ");
            self.deref().draw()
        }
    }

    let shapes1: [Box<dyn Shape>; 2] = [
        Box::new(Circle { radius: 99 }),
        Box::new(Rectangle { length: 100, width: 101, }),
    ];

    for shape in shapes1 { shape.draw(); }

    let shapes2: [&dyn Shape; 2] = [
        &Circle { radius: 200 },
        &Rectangle { length: 201, width: 202, },
    ];

    // Doesn't match the expected impl:
    for shape in shapes2 { shape.draw(); }
}
