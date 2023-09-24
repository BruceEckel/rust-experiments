use std::thread;
use std::time::Instant;

<<<<<<< HEAD
pub async fn rand_int(rng: &mut StdRng, yielding: bool) -> i32 {
    if yielding {
        tokio::task::yield_now().await; // Allow context switch
    }
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
=======
pub fn cpu_bound_task(name: &str, iterations: usize) {
    let current_thread = thread::current();
    println!("{} starts on thread: {:?} (id: {:?})", name, current_thread.name().unwrap_or("[unnamed]"), current_thread.id());
>>>>>>> parent of 471892c (random generator with yield, run_tasks())
    let start = Instant::now();

    let mut x = 0;
<<<<<<< HEAD
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
            let future = async move { rand_int(&mut cloned_rng, true).await };
            Box::pin(future)
        }));
        let task_two = tokio::spawn(calculation("two", |rng| {
            let mut cloned_rng = rng.clone();
            let future = async move { rand_int(&mut cloned_rng, true).await };
            Box::pin(future)
        }));

        // Await the completion of both tasks
        let _ = tokio::try_join!(task_one, task_two);

        println!("Duration: {:?}", start.elapsed());
    });
=======
    for _ in 0..iterations {
        x += 1;
    }

    println!("{} ends after {:?}: {}", name, start.elapsed(), x);
>>>>>>> parent of 471892c (random generator with yield, run_tasks())
}
