fn unit_return() -> ()  {}
fn no_return() {}

fn main() {
    assert!(unit_return() == ());
    assert!(no_return() == ());
    assert!(unit_return() == no_return());
}
