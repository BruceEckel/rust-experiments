#![allow(unused)]
fn main() {
    let s1 = &[1, 2, 3, 99, 878];
    let s2 = &[4, 5, 6];

    let mut iter = s1.iter().zip(s2);
    println!("s1: {:?}", s1);
    println!("s2: {:?}", s2);
    println!("iter: {:?}", iter.clone().collect::<Vec<_>>());

    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    assert_eq!(iter.next(), Some((&3, &6)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    let enumerate: Vec<_> = "foo".chars().enumerate().collect();

    let zipper: Vec<_> = (0..).zip("foo".chars()).collect();

    println!("enumerate: {:?}", enumerate);
    println!("zipper: {:?}", zipper);
}
