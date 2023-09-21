#[allow(dead_code)]
mod cpu_task;
use cpu_task::consume_numbers;

#[tokio::main]
async fn main() {
    let producer_count = 1_000_000;
    let sum = consume_numbers(producer_count).await;
    println!("Sum of {} numbers: {}", producer_count, sum);
}
