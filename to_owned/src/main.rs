use std::collections::HashMap;

fn main() {
  // Changed: HashMap<&str, u32> -> HashMap<String, u32>
  let mut words: HashMap<String, u32> = HashMap::new();

  loop {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    for word in input.split_whitespace() {
      let count = match words.get(word) {
        None => 0,
        Some(count) => *count,
      };
      // Changed: word -> word.to_owned()
      // words.insert(word.to_owned(), count + 1);
      // Same as:
      words.insert(String::from(word), count + 1);
    }
    println!("{:?}", words);
  }
}
