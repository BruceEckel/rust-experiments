#[derive(Debug)] // just so we can print out User
#[allow(dead_code)]
struct User {
    id: u32,
}

// fn main() {
//   let u1 = User{id: 9000};
//   println!("{:#?}", u1);

//   let u2 = u1;
//   println!("{:#?}", u2);

//   // this is an error
//   println!("{:#?}", u2);
// }

fn print_user(u: User) -> User {
    println!("{:?}", u);
    u;
}

fn main() {
    let u = User { id: 9000 };
    let u = print_user(u);

    // this isn an error
    println!("{:?}", u);
}
