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
}
/*
auto_overload(overloading::Dog)
auto_overload(overloading::Cat)
auto_overload(overloading::Giraffe)
dynamic(&dyn overloading::Animal)
 */
