use shared_memory::*;
use std::future::Future;
use std::pin::Pin;
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

struct FutexTest<'a> {
    ptr: Arc<&'a mut AtomicU8>,
    woke: Arc<AtomicBool>,
}

impl FutexTest<'static> {
    pub fn new(value: i32) -> Self {
        let mut shmem = match ShmemConf::new().size(4096).os_id("RUST_TEST").create() {
            Ok(m) => m,
            Err(ShmemError::MappingIdExists) => ShmemConf::new().os_id("RUST_TEST").open().unwrap(),
            Err(e) => {
                println!("Unable to create or open shmem flink {} : {}", "test", e);
                unsafe {
                    libc::exit(1);
                }
            }
        };

        shmem.set_owner(false);
        println!("SHMEM OS_ID: {}", shmem.get_os_id());

        let raw_ptr = shmem.as_ptr();
        unsafe {
            *raw_ptr = 0;
        }

        let p = unsafe { Arc::new(&mut *(raw_ptr as *mut u8 as *mut AtomicU8)) };

        Self {
            ptr: p,
            woke: Arc::new(AtomicBool::new(false)),
        }
    }
}
impl Future for FutexTest<'static> {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if (*self.woke).load(Ordering::Relaxed) {
            return Poll::Ready("done");
        }
        let waker = cx.waker().clone();
        let mut woke = self.woke.clone();
        let mut p = self.ptr.clone();
        println!("poll");
        thread::spawn(move || {
            let mut shmem = ShmemConf::new().os_id("RUST_TEST").open().unwrap();
            shmem.set_owner(true);
            let raw_ptr = shmem.as_ptr();
            unsafe {
                let null: *const i32 = ptr::null();
                libc::syscall(libc::SYS_futex, raw_ptr, libc::FUTEX_WAIT, 0, null, null, 0);
            }
            println!("Futex Bumped");
            (*woke).store(true, Ordering::Relaxed);
            waker.wake();
        });

        Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    let future = FutexTest::new(0);
    thread::spawn(move || {
        let mut shmem = ShmemConf::new().os_id("RUST_TEST").open().unwrap();
        shmem.set_owner(false);
        thread::sleep(Duration::from_millis(500));
        let raw_ptr = shmem.as_ptr();
        let p = unsafe { Arc::new(&mut *(raw_ptr as *mut u8 as *mut AtomicU8)) };
        p.store(1, Ordering::Relaxed);
        unsafe {
            let null: *const i32 = ptr::null();
            libc::syscall(
                libc::SYS_futex,
                raw_ptr,
                libc::FUTEX_WAKE,
                libc::INT_MAX,
                null,
                null,
                0,
            );
        }
        println!("wakeup");
    });

    let out = future.await;
    assert_eq!(out, "done");
}
