// Derived from https://github.com/rust-lang/rust/issues/66359
// "lets you query the type name of an unusable binding"

macro_rules! type_name_of {
    ( $e:expr $(,)? ) => {{
        let it = [];
        #[allow(unreachable_code)]
        {
            if false {
                loop {}
                (&mut { it })[0] = &$e
            }
        }
        $crate::__helper__(it)
    }};
}

// pub(crate) use type_name_of;

// #[doc(hidden)]
pub fn __helper__<T>(_: [&T; 0]) -> &'static str {
    ::core::any::type_name::<T>()
}

fn main() {
    let it = (vec![()], 42);
    drop(it.0);
    dbg!(type_name_of!(it));

    fn f(_i: i32) {}
    dbg!(type_name_of!(f));
}
/*
[src\main.rs:28] type_name_of!(it) = "(alloc::vec::Vec<()>, i32)"
[src\main.rs:31] type_name_of!(f) = "type_name_of::main::f"
 */
