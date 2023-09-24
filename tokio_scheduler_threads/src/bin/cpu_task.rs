use std::thread;
use std::time::Instant;

pub async fn rand_int(rng: &mut StdRng, yielding: bool) -> i32 {
    if yielding {
        tokio::task::yield_now().await; // Allow context switch
    }
    rng.gen_range(0..100)
}

pub async fn calculation(name: &str, yielding: bool) {
    let current_thread = thread::current();
    println!(
        "{} starts on thread {:?} (id: {:?})",
        name,
        current_thread.name().unwrap_or("[unnamed]"),
        current_thread.id()
    );
    let start = Instant::now();

    let mut x = 0;
    for _ in 0..1_000_000 {
        x += (rand_int(&mut rng, yielding)).await;
    }

    println!("Task {} ends after {:?}: {}", name, start.elapsed(), x);
}

pub fn run_tasks(rt: Runtime) {
    rt.block_on(async {
        let start = Instant::now();

        let task_one = tokio::spawn(calculation("one", true));
        let task_two = tokio::spawn(calculation("two", true));

        // Await the completion of both tasks
        let _ = tokio::try_join!(task_one, task_two);

        println!("Duration: {:?}", start.elapsed());
    });
    println!("{} ends after {:?}: {}", name, start.elapsed(), x);
}
