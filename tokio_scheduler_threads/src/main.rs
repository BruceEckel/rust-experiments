use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::ops::Range;
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use tokio::runtime::{Builder, Runtime};

// Upper & lower percent AND random range:
const SPAN: Range<i32> = 0..100;

#[derive(Copy, Clone)]
pub struct YieldPercent {
    pub value: i32,
}

impl YieldPercent {
    pub fn new(value: i32) -> Self {
        Self {
            value: value
                .clamp(SPAN.start, SPAN.end),
        }
    }
    pub fn list(values: &[i32]) -> Vec<Self> {
        values
            .iter()
            .map(|&value| Self::new(value))
            .collect()
    }
}

pub async fn rand_int(
    rng: &mut StdRng,
    yield_percent: &YieldPercent,
) -> i32 {
    let random = rng.gen_range(SPAN);
    // Probability-based context switch:
    if random < yield_percent.value {
        tokio::task::yield_now().await;
    }
    random
}

pub async fn calculation(
    name: &str,
    yield_percent: Arc<YieldPercent>,
) {
    println!(
        "\nStart '{}' with yield_percent {}",
        name, yield_percent.value
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
        x += rand_int(
            &mut rng,
            &*yield_percent,
        )
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
    yield_percent: YieldPercent,
) {
    let start = Instant::now();
    let yield_percent =
        Arc::new(yield_percent);
    rt.block_on(async {
        let task_one =
            tokio::spawn(calculation(
                "one",
                yield_percent.clone(),
            ));
        let task_two =
            tokio::spawn(calculation(
                "two",
                yield_percent.clone(),
            ));
        let _ = tokio::try_join!(
            task_one, task_two
        );
    });
    println!(
        "=> Elapsed: {:?}",
        start.elapsed()
    );
}

fn main() {
    let yields = YieldPercent::list(&[
        0, 5, 25, 50, 75, 100,
    ]);
    println!("Single-threaded tokio async");
    let rts = Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    for yld in &yields {
        run_tasks(&rts, *yld);
    }

    println!("\nTwo-threaded tokio async");
    let rtm = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .build()
        .unwrap();
    for yld in &yields {
        run_tasks(&rtm, *yld);
    }
}
