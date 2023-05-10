#![allow(dead_code, unused_variables)]

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

pub struct Join<FutureA, FutureB> {
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        // If self.a is not None, then fa has not yet been run.
        if let Some(fa) = &mut self.a {
            // Calling poll() starts the fa task.
            // In doing so we've passed the outer wake() to fa's poll().
            // We never call poll() again (???), so is that the best name?
            // If the call to poll() on fa produces Ready, then the task
            // has completed and is ready to deliver results.
            // (But where the heck does "take()" come from?)
            if let Poll::Ready(()) = fa.poll(wake) {
                self.a.take();
            }
        }

        // Same logic for element b inside struct Join.
        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        // Similar question: where did is_none() come from?
        if self.a.is_none() && self.b.is_none() {
            // Both futures have completed -- we can return successfully
            Poll::Ready(())
        } else {
            // One or both futures returned `Poll::Pending` and still have
            // work to do. They will call `wake()` when progress can be made.
            Poll::Pending
            // Note that each future is responsible, internally, for calling
            // the wake() function WHEN something happens. If you are running
            // on an operating system, this would presumably come from the
            // state changing around IO. If you are NOT running on an OS,
            // what (for example) wakes up the hardware if it is asleep can
            // be the same thing that indicates that runs the wake() function.

            // There is no actual polling, which means that this approach
            // removes the need for an event loop. Again, ideal for battery-run
            // embedded systems.
        }
    }
}

fn main() {
    println!("Hello, world!");
}
