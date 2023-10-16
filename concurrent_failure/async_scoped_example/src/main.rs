use tokio;

#[tokio::main]
async fn main() {
    let not_copy = String::from("hello world!");
    let not_copy_ref = &not_copy;
    let (foo, outputs) = async_scoped::TokioScope::scope_and_block(|s| {
        for _ in 0..10 {
            let proc = || async {
                assert_eq!(not_copy_ref, "hello world!");
                eprintln!("Hello world!")
            };
            s.spawn(proc());
        }
        42
    });
    assert_eq!(foo, 42);
    assert_eq!(outputs.len(), 10);
}
