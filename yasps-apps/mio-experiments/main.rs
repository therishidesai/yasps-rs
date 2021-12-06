use anyhow::Result;
use mio::unix::SourceFd;
use mio::{Events, Interest, Poll, Token};

use std::collections::HashMap;
use std::ptr::addr_of_mut;

pub fn main() -> Result<()> {
    let efd = unsafe { libc::eventfd(0, libc::EFD_NONBLOCK) };

    let mut mio_efd = SourceFd(&efd);
    std::thread::spawn(move || {
        for i in 0..5 {
            unsafe {
                let mut c = i;
                libc::write(efd, &mut c as *mut _ as _, 8);
                libc::sleep(1 as u32);
            }
        }
    });

    let mut poll = Poll::new()?;

    let mut events = Events::with_capacity(1024);

    let mut efds = HashMap::new();

    efds.insert(Token(0), efd);
    // Register the listener
    poll.registry()
        .register(&mut mio_efd, Token(0), Interest::READABLE)?;

    let mut leave = false;
    // Wait for the eventfd to become ready. This has to happens in a loop to
    // handle spurious wakeups.
    while !leave {
        poll.poll(&mut events, None)?;

        for event in &events {
            if event.token() == Token(0) && event.is_readable() {
                // The socket connected (probably, it could still be a spurious
                // wakeup)
                println!("event on eventfd!");
                unsafe {
                    let mut count = 0;
                    let event_fd = efds.get(&event.token()).unwrap();
                    if libc::read(*event_fd, addr_of_mut!(count) as _, 8) < 0 {
                        panic!("read fail");
                    }
                    println!("success read from efd, read bytes({})", count);
                    if count == 4 {
                        leave = true;
                    }
                }
            }
        }
    }

    unsafe {
        println!("closing efd");
        libc::close(efd);
    };

    Ok(())
}
