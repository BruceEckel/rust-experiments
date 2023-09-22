use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::thread;
use std::time::Instant;
use tokio::runtime::Runtime;

pub async fn rand_int_yield(rng: &mut StdRng) -> i32 {
    tokio::task::yield_now().await; // Allow context switch
    rng.gen_range(0..100)
}

pub async fn rand_int_no_yield(rng: &mut StdRng) -> i32 {
    // No context switch
    rng.gen_range(0..100)
}

pub async fn calculation<F>(name: &str, mut rand_func: F)
where
    F: FnMut(&mut StdRng) -> std::pin::Pin<Box<dyn std::future::Future<Output = i32> + Send>>,
{
    let current_thread = thread::current();
    println!(
        "Task {} starts on thread {:?} (id: {:?})",
        name,
        current_thread.name().unwrap_or("[unnamed]"),
        current_thread.id()
    );
    let start = Instant::now();
    let mut rng: StdRng = SeedableRng::from_seed([42; 32]);

    let mut x = 0;
    for _ in 0..1_000_000 {
        x += (rand_func(&mut rng)).await;
    }

    println!("Task {} ends after {:?}: {}", name, start.elapsed(), x);
}

pub fn run_tasks(rt: Runtime) {
    rt.block_on(async {
        let start = Instant::now();

        let task_one = tokio::spawn(calculation("one", |rng| {
            let mut cloned_rng = rng.clone();
            let future = async move { rand_int_yield(&mut cloned_rng).await };
            Box::pin(future)
        }));
        let task_two = tokio::spawn(calculation("two", |rng| {
            let mut cloned_rng = rng.clone();
            let future = async move { rand_int_no_yield(&mut cloned_rng).await };
            Box::pin(future)
        }));

        // Await the completion of both tasks
        let _ = tokio::try_join!(task_one, task_two);

        println!("Duration: {:?}", start.elapsed());
    });
}
