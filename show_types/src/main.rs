#![allow(unused_variables, dead_code)]

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    let int_var = 42;
    let float_var = 3.14;
    let string_literal = "Hello, World!";
    let string_var = String::from("Hello, World!");
    let tuple_var = (42, 3.14, "Hello, World!");
    let array_var = [1, 2, 3, 4, 5];
    let vector_var = vec![1, 2, 3, 4, 5];

    dbg!(type_of(&int_var));
    dbg!(type_of(&float_var));
    dbg!(type_of(&string_literal));
    dbg!(type_of(&string_var));
    dbg!(type_of(&tuple_var));
    dbg!(type_of(&array_var));
    dbg!(type_of(&vector_var));

    struct Foo(i32, String);
    dbg!(type_of(&Foo(1, format!("one"))));

    fn f() {}
    dbg!(type_of(&f));
    fn g(i:i32, j: &str) {}
    dbg!(type_of(&g));
}
/* Functions don't include argument type information:
[src\main.rs:16] type_of(&int_var) = "i32"
[src\main.rs:17] type_of(&float_var) = "f64"
[src\main.rs:18] type_of(&string_literal) = "&str"
[src\main.rs:19] type_of(&string_var) = "alloc::string::String"
[src\main.rs:20] type_of(&tuple_var) = "(i32, f64, &str)"
[src\main.rs:21] type_of(&array_var) = "[i32; 5]"
[src\main.rs:22] type_of(&vector_var) = "alloc::vec::Vec<i32>"
[src\main.rs:25] type_of(&Foo(1, format!("one"))) = "show_types::main::Foo"
[src\main.rs:28] type_of(&f) = "show_types::main::f"
[src\main.rs:30] type_of(&g) = "show_types::main::g"
 */
