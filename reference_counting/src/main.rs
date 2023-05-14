#![allow(unused)]
use std::rc::Rc;

// Instead implement custom formatter:
// https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html

#[derive(Debug)]
struct Owner(String);

#[derive(Debug)]
struct Gadget {
    id: i32,
    owner: Rc<Owner>,
    // ...other fields
}

fn main() {
    // Create a reference-counted `Owner`.
    let gadget_owner: Rc<Owner> = Rc::new(
        Owner(format!("Gadget Man")),
    );

    // Create `Gadget`s belonging to `gadget_owner`. Cloning the `Rc<Owner>`
    // gives us a new pointer to the same `Owner` allocation, incrementing
    // the reference count in the process.
    let gadget1 = Gadget {
        id: 1,
        owner: Rc::clone(&gadget_owner),
    };
    let gadget2 = Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner),
    };

    // Dispose of our local variable `gadget_owner`.
    drop(gadget_owner);

    // Despite dropping `gadget_owner`, we're still able to print out the name
    // of the `Owner` of the `Gadget`s. This is because we've only dropped a
    // single `Rc<Owner>`, not the `Owner` it points to. As long as there are
    // other `Rc<Owner>` pointing at the same `Owner` allocation, it will remain
    // live. The field projection `gadget1.owner.name` works because
    // `Rc<Owner>` automatically dereferences to `Owner`.
    println!("{gadget1:?}");
    println!("{gadget2:?}");

    // At the end of the function, `gadget1` and `gadget2` are destroyed, and
    // with them the last counted references to our `Owner`. Gadget Man now
    // gets destroyed as well.
}
