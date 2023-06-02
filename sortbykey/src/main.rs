#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn rect(width: u32, height: u32) -> Rectangle {
    Rectangle { width: width, height: height }
}

fn main() {
    let mut list = [ rect(10, 1), rect(3, 5), rect(7, 12), ];
    //     Rectangle { width: 10, height: 1 },
    //     Rectangle { width: 3, height: 5 },
    //     Rectangle { width: 7, height: 12 },
    // ];
    println!("{:#?}", list);
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
