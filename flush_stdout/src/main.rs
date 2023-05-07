use std::io::stdout;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    let interval = Duration::new(0, 150_000_000);
    for x in 17u8..50 {
        thread::sleep(interval);
        print!("\r{x} = {}", x as char);
        stdout().flush().unwrap();
    }
}
