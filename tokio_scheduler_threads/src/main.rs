use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::thread;
use std::time::Instant;
use tokio::runtime::Builder;
use tokio::runtime::Runtime;

pub async fn rand_int(
    rng: &mut StdRng,
    yielding: bool,
) -> i32 {
    if yielding {
        // Allow context switch
        tokio::task::yield_now().await;
    }
    rng.gen_range(0..100)
}

pub async fn calculation(
    name: &str,
    yielding: bool,
) {
    println!(
        "\nStarting '{}' with yielding {}",
        name, yielding
    );
    let current_thread = thread::current();
    println!(
        "'{}' on thread {:?} (id: {:?})",
        name,
        current_thread.name().unwrap_or("X"),
        current_thread.id()
    );
    let start = Instant::now();
    let mut rng: StdRng =
        SeedableRng::from_seed([42; 32]);
    let mut x = 0;
    for _ in 0..1_000_000 {
        x += (rand_int(&mut rng, yielding))
            .await;
    }
    println!(
        "Task '{}' ends after {:?}: {}",
        name,
        start.elapsed(),
        x
    );
}

pub fn run_tasks(
    rt: &Runtime,
    yielding: bool,
) {
    let start = Instant::now();
    rt.block_on(async {
        let task_one = tokio::spawn(
            calculation("one", yielding),
        );
        let task_two = tokio::spawn(
            calculation("two", yielding),
        );
        let _ = tokio::try_join!(
            task_one, task_two
        );
    });
    println!("=> Elapsed: {:?}", start.elapsed());
}

fn main() {
    println!("Single-threaded tokio async");
    let rts = Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    run_tasks(&rts, false);
    run_tasks(&rts, true);

    println!("\nMulti-threaded tokio async");
    let rtm = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .build()
        .unwrap();
    run_tasks(&rtm, false);
    run_tasks(&rtm, true);
}
