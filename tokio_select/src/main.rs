use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
    let mut stream1 = stream::iter("abcdefg".chars());
    let mut stream2 = stream::iter("hijklmnop".chars());
    let mut stream3 = stream::iter("qrstuv".chars());
    let mut stream4 = stream::iter("wxyz".chars());

    let mut values = vec![];

    loop {
        tokio::select! {
            Some(v) = stream1.next() => values.push(v),
            Some(v) = stream2.next() => values.push(v),
            Some(v) = stream3.next() => values.push(v),
            Some(v) = stream4.next() => values.push(v),
            else => break,
        }
    }
    let mut result = String::new();
    for c in values {
        result.push(c);  // Must be a more direct way to do this...
    }
    println!("{result} {}", result.len());
}