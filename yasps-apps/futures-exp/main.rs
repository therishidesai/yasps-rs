use linux_futex::{Futex, Private};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

struct FutexTest {
    f: Arc<Box<Futex<Private>>>,
    woke: Arc<AtomicBool>,
}

impl Future for FutexTest {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if (*self.woke).load(Ordering::Relaxed) {
            return Poll::Ready("done");
        }
        let waker = cx.waker().clone();
        let f = self.f.clone();
        let mut woke = self.woke.clone();
        println!("poll");
        thread::spawn(move || {
            (*f).wait(0);
            println!("Futex Bumped");
            (*woke).store(true, Ordering::Relaxed);
            waker.wake();
        });

        Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    let f: Futex<Private> = Futex::new(0);

    let bf = Arc::new(Box::new(f));
    let bw = Arc::new(AtomicBool::new(false));
    let future = FutexTest {
        f: bf.clone(),
        woke: bw,
    };

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        println!("wakeup");
        (*bf).wake(1);
    });

    let out = future.await;
    assert_eq!(out, "done");
}
