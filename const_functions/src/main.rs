#[derive(Debug)]
struct Thing(i32);

// All calculations are performed at compile time.
// The code for this function is not included in the binary.
const fn const_fn(a: Thing, b: Thing) -> Thing {
    Thing(a.0 + b.0)
}

// Mutable references are not allowed in constant functions:
// const fn non_const() -> i32 {
fn non_const() -> i32 {
    use rand::*;
    rand::thread_rng().gen_range(1..=100)
}

fn regular_fn_ref_args(a: &Thing, b: &Thing) -> Thing {
    Thing(a.0 + b.0)
}

const fn const_fn_ref_args(a: &Thing, b: &Thing) -> Thing {
    Thing(a.0 + b.0)
}

fn main() {
    let (x, y) = (Thing(42), Thing(55));
    dbg!(const_fn(x, y));
    // const_fn(x,y);  // Can't do this because x & y were moved in prev call

    let (u, v) = (Thing(9), Thing(13));
    dbg!(regular_fn_ref_args(&u, &v));
    regular_fn_ref_args(&u, &v);

    let (i, j) = (Thing(17), Thing(29));
    dbg!(const_fn_ref_args(&i, &j));
    const_fn_ref_args(&i, &j);

    dbg!(non_const());
}
