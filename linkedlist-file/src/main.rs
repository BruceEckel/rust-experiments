use std::fs::File;
use std::io::{BufRead, BufReader};

enum LinkedList<T> {
    Node(T, Box<LinkedList<T>>),
    Nil,
}

use LinkedList::*;

fn main() {
    // Open and read a text file
    if let Ok(file) = File::open("input.txt") {
        let reader = BufReader::new(file);

        // Create an empty linked list
        let mut list = Box::new(Nil);

        // Iterate over lines in the file and add words to the linked list
        for line in reader.lines() {
            if let Ok(line) = line {
                let words: Vec<&str> = line.split_whitespace().collect();
                for word in words {
                    list = Box::new(Node(word.to_string(), list));
                }
            }
        }

        // Print the elements in the linked list
        print_linked_list(&list);
    } else {
        println!("Failed to open the file.");
    }
}

fn print_linked_list<T: std::fmt::Display>(list: &LinkedList<T>) {
    match list {
        Node(value, next) => {
            println!("Value: {}", value);
            print_linked_list(next);
        }
        Nil => println!("End of list"),
    }
}
