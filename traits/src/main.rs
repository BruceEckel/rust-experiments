#![allow(dead_code)]

#[test]
fn normal_trait() {
    struct SomeType {}
    trait MyTrait {}
    impl MyTrait for SomeType {}
    // impl MyTrait for SomeType {}  // "conflicting implementations"
}

#[test]
fn _traits_as_types() {
    trait MyTrait {}
    // A "trait object" must use `dyn`:
    impl dyn MyTrait {}
    impl dyn MyTrait {} // OK
    impl dyn MyTrait {
        fn foo() {}
    }
    impl dyn MyTrait {
        fn bar() {}
    }
}

#[test]
fn _trait_objects_for_trait_objects() {
    trait MyTrait {}
    impl dyn MyTrait {} // Must use dyn
    trait AnotherTrait {}
    // ... Except when it's coming from another trait object?:
    impl AnotherTrait for dyn MyTrait {}
}

#[test]
#[allow(dead_code)]
fn draw() {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }
}

#[test]
fn _trait_objects_and_dyn() {
    // define an example struct, make it printable
    #[derive(Debug)]
    struct Foo;

    // an example trait
    trait Bar {
        fn baz(&self);
    }

    // implement the trait for Foo
    impl Bar for Foo {
        fn baz(&self) {
            println!("{:?}", self)
        }
    }

    // This is a generic function that takes any T that implements trait Bar.
    // It must resolve to a specific concrete T at compile time.
    // The compiler creates a different version of this function
    // for each concrete type used to call it so &T here is NOT
    // a trait object (as T will represent a known, sized type
    // after compilation)
    fn static_dispatch<T>(t: &T)
    where
        T: Bar,
    {
        t.baz(); // we can do this because t implements Bar
    }

    // This function takes a pointer to a something that implements trait Bar
    // (it'll know what it is only at runtime). &dyn Bar is a trait object.
    // There's only one version of this function at runtime, so this
    // reduces the size of the compiled program if the function
    // is called with several different types vs using static_dispatch.
    // However performance is slightly lower, as the &dyn Bar that
    // dynamic_dispatch receives is a pointer to the object +
    // a vtable with all the Bar methods that the object implements.
    // Calling baz() on t means having to look it up in this vtable.
    fn dynamic_dispatch(t: &dyn Bar) {
        // ----------------^
        // this is the trait object! It would also work with Box<dyn Bar> or
        // Rc<dyn Bar> or Arc<dyn Bar>
        //
        t.baz(); // we can do this because t implements Bar
    }

    let foo = Foo;
    static_dispatch(&foo);
    dynamic_dispatch(&foo);
}

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Sheep {
        Sheep {
            name: name,
            naked: false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name(), self.noise());
    }
}

#[derive(Debug)]
struct Seconds(i32, String);

fn own(s: &Seconds) {
    println!("{:?}", s);
}

#[test]
fn sheep() {
    // Type annotation is required here:
    let mut dolly: Sheep = Animal::new("Dolly");
    let secs = Seconds(50, "long".to_string());
    println!("{:?}", secs);
    own(&secs);
    println!("{:?}", secs);
    dolly.talk();
    dolly.shear();
    dolly.talk();
}

fn main() {}
