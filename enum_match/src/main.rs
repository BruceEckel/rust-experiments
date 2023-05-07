#![allow(dead_code, unused_variables)]

#[derive(Debug)]
struct Foo; // Semicolons required

#[derive(Debug)]
struct Bar(i32); // A "named tuple", a specific kind of struct

#[derive(Debug)]
struct Baz {
    n: i32,
    b: i64,
} // Can't have a semicolon (Never a semicolon after a brace?)

fn main() {
    #[derive(Debug)]
    enum Poll2<T> {
        Pending,
        // 'struct' is implied inside an enum:
        Ready(T),               // Named tuple
        Thing { n: i32, s: T }, // Regular struct
    }
    use Poll2::*; // Bring enum elements into current namespace
    let p1 = Ready(11); // : Poll2<i32>
    let p2 = Ready("hello".to_string()); // : Poll2<String>
    let p3 = Thing {  // : Poll2<String>
        n: 42,
        s: "Thing String".to_string(),
    };
    fn check<T: std::fmt::Debug>(p: Poll2<T>) {
        let t_type = std::any::type_name::<T>();
        match p {
            Pending => println!("Pending<{}>", t_type),
            Ready(x) => println!("Ready<{}>: {:?}", t_type, x),
            Thing { n, s } => println!("Thing<{}>: {}, {:?}", t_type, n, s),
        }
    }
    check(Poll2::<i32>::Pending);
    check(Poll2::<String>::Pending);
    check(Poll2::<Foo>::Pending);
    check(Poll2::<Bar>::Pending);
    check(Poll2::<Baz>::Pending);
    check(p1);
    check(p2);
    check(p3);
    check(Poll2::<Bar>::Ready(Bar(11)));
    check(Poll2::<Bar>::Thing { n: 99, s: Bar(11) });
    check(Poll2::<Baz>::Ready(Baz { n: 42, b: 47 }));
    check(Poll2::<Baz>::Thing {
        n: 99,
        s: Baz { n: 42, b: 47 },
    });
}
