fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2 + 2)
}

fn main() {
    let mut t = (1,2,3);
    t.1 = 100;
    println!("{:?}", t);

    let singles = (-10..10).collect();
    let doubles = double_positives(&singles).collect::<Vec<i32>>();
    println!("{:?}", doubles);
    assert_eq!(doubles, vec![4, 6, 8, 10, 12, 14, 16, 18, 20]);
}
