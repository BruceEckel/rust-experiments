fn main() {
    let mut i: i32 = 1;
    fn change(n: &mut i32) -> &mut i32 {
        *n += 1;
        n
    }
    dbg!(change(&mut i));
    let mut x = i;
    dbg!(x, i);
    dbg!(change(&mut x));
    dbg!(x, i);

    // fn foop(n: mut i32) {}  // Illegal without the '&'

    fn fip(n: i32) -> i32 {
        n
    }
    dbg!(fip(x));  // Immutable accepts mutable
}
