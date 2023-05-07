struct A;
struct Blob(i32, i32);

fn get(b: Blob) -> (i32, i32) {
    // println!("{}", b.2); // error[E0609]: no field `2` on type `Blob`
    (b.0, b.1)
}

fn main() {
    let _a = A; // No braces required
    println!("Hello, world!");
    let x = Blob(1, 2);
    let (u, v) = get(x);
    println!("{}, {}", u, v);
}
