// Polymorphism: dyn vs. impl
#![allow(dead_code, unused_variables)]

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

// `&` because size is unknown, `dyn` because type is unknown.
// `dyn` produces a "fat pointer" including a vtable.
fn show_dyn(s: &dyn Shape) {
    s.draw();
}
// Shape isn't automatically burdened with a vtable!

// Like C++ templates--new version generated for each argument type:
fn show_impl(s: impl Shape) {
    s.draw();
}
// Invisible automatically-generated function overloading!

fn main() {
    const SHAPES: [&dyn Shape; 2] = [
        &Circle { radius: 1 },
        &Rectangle { length: 2, width: 3, },
    ];
    SHAPES[0].draw();
    SHAPES[1].draw();
    for shape in SHAPES {
        show_dyn(shape);
        // Doesn't know the type at compile time, so it can't generate:
        // show_impl(shape);
    }
    let (c, r) = (
        Circle { radius: 4 },
        Rectangle { length: 5, width: 6, },
    );
    show_impl(c); // Creates a version of show_impl() for Circle
    show_impl(r); // Creates a version of show_impl() for Square
    // show_impl(*SHAPES[0]);  // Doesn't know the type

    vec_of_shapes();
}

fn vec_of_shapes() {  // More advanced...
    struct Shapes<'a> {
        shapes: Vec<Box<&'a dyn Shape>>,
    }

    impl Shapes<'_> {
        fn new() -> Shapes<'static> {
            Shapes {
                shapes: vec![
                    Box::new(&Circle { radius: 11 }),
                    Box::new(&Rectangle { length: 12, width: 13, }),
                    Box::new(&Circle { radius: 14 }),
                    Box::new(&Rectangle { length: 15, width: 16, }),
                ]
            }
        }
    }

    let shapes: Vec<Box<&dyn Shape>> = vec![
            Box::new(&Circle { radius: 11 }),
            Box::new(&Rectangle { length: 12, width: 13, }),
            Box::new(&Circle { radius: 14 }),
            Box::new(&Rectangle { length: 15, width: 16, }),
        ];

    for shape in Shapes::new().shapes.iter() {
        show_dyn(**shape);
    }
    for shape in shapes.iter() {
        show_dyn(**shape);
    }
}

