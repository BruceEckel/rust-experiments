fn main() {
    let empty: Vec<&str> = vec![];
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(n) => println!("got: {n:?} things"),
            _ => (),
        }
        let words: Vec<&str> = input.split_whitespace().collect();
        if words == empty {
            return;
        }
        println!("{:?}", words);
    }
}
