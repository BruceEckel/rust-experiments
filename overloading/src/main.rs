struct Dog;
struct Cat;
struct Giraffe;

trait Animal {}
impl Animal for Dog {}
impl Animal for Cat {}
impl Animal for Giraffe {}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn auto_overload(a: impl Animal) {
    println!("auto_overload({})", type_of(&a));
}

fn main() {
    auto_overload(Dog {});
    auto_overload(Cat {});
    auto_overload(Giraffe {});

    let base_animal: &dyn Animal = &Giraffe {};
    // auto_overload(base_animal);  // Nope
    fn dynamic(a: &dyn Animal) {
        println!("dynamic({})", type_of(&a));
    }
    dynamic(base_animal);

    println!("dynamic as *const (): {:p}", dynamic as *const ());
    // println!("auto_overload as *const (): {:p}", auto_overload::<Dog> as *const ());
    // The previous line produces:
    // error[E0282]: type annotations needed
    //   --> src\main.rs:31:50
    //    |
    // 31 |     println!("auto_overload as *const (): {:p}", auto_overload as *const ());
    //    |                                                  ^^^^^^^^^^^^^ cannot infer type for type parameter `impl Animal` declared on the function `auto_overload`

    // What would a correcting type annotation look like?
    // (Apparently this cannot be done...)

    // Shows that generics create a different function for each type parameter:
    fn generic_address<A: Animal>(a: A) {
        println!("generic_address({})", type_of(&a));
    }
    dbg!(generic_address::<Dog> as *const ());
    dbg!(generic_address::<Cat> as *const ());
    dbg!(generic_address::<Giraffe> as *const ());
}
