#![allow(unused)]

use std::io::Write;
use std::io::stdout;

fn main() {
    let mut array /* : [i32; 5] */ = [74, 22, 9, 42, 33];
    fn show(mut array: [i32; 5]) {
        for (i, elem) in array.iter_mut().enumerate() {
            print!("{i}: {elem}, ");
        }
        println!();
    };
    show(array);
    dbg!(&array[3..]);
    array[1] = 1;
    array[2] = 2;
    show(array);

    assert_eq!([2, 42, 33], &array[2..]);
    dbg!(array);

    let immutable = [74, 22, 9, 42, 33];
    show(immutable); // show() works with immutable arg
    t2021();
    array_of_enums();
}

fn t2021() {
    let array: [i32; 4] = [99, 42, 24, 979]; // Immutable

    // Iterate by reference:
    for item in array.iter().enumerate() {
        let (i, x): (usize, &i32) = item;
        println!("array[{i}]: {x}");
    }

    // Iterate by value:
    for item in array.into_iter().enumerate() {
        let (i, x): (usize, i32) = item;
        println!("array[{i}]: {x}");
    }

    array.iter().map(|e| print!("{e}, ")).collect::<Vec<_>>();
    stdout().flush().unwrap();
}

fn array_of_enums() {
    #[derive(Debug)]
    enum Color {
        Red,
        Blue,
        Green,
        Purple,
    }
    use Color::*;
    let aoe: [Color; 4] = [Red, Blue, Green, Purple];
    dbg!(aoe);
}
