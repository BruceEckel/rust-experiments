use std::fmt;

struct Fub(String);

// To use `{}` requires `fmt::Display`
impl fmt::Display for Fub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write into the output stream `f`.
        // `write!` syntax is similar to `println!` and `format!`.
        write!(f, "Fub display: {}", self.0)
    }
}

fn main() {
    println!("{}", Fub(format!("Hola!")));
}
