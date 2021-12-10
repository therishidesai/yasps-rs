use libc::{c_char, c_void, off_t, size_t};
use libc::{close, ftruncate, memcpy, mmap, shm_open, strncpy};
use libc::{MAP_SHARED, O_CREAT, O_RDWR, PROT_WRITE, S_IRUSR, S_IWUSR};
use std::error::Error;
use std::process::Command;
use std::{env, ptr, str};

const STORAGE_ID: *const c_char = b"MY_MEM_ID\0".as_ptr() as *const c_char;
const STORAGE_SIZE: size_t = 128;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();

    let (fd, addr) = unsafe {
        let null = ptr::null_mut();
        let fd = shm_open(STORAGE_ID, O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);
        let _res = ftruncate(fd, STORAGE_SIZE as off_t);
        let addr = mmap(null, STORAGE_SIZE, PROT_WRITE, MAP_SHARED, fd, 0);

        (fd, addr)
    };
    if args.len() == 1 {
        // Consumer...

        let exec_path = &args[0];

        // Start producer process. Block until done.
        let output = Command::new(exec_path).arg("Producer...").output()?;

        println!("Producer stdout  : {}", str::from_utf8(&output.stdout)?);

        let mut data = [0_u8; STORAGE_SIZE];
        let pdata = data.as_mut_ptr() as *mut c_char;

        unsafe {
            strncpy(pdata, addr as *const c_char, STORAGE_SIZE);
            close(fd);
        }
        println!("Producer message : {}", str::from_utf8(&data)?);
    } else {
        // Producer...

        let data = b"Hello, World!\0";
        let pdata = data.as_ptr() as *const c_void;

        unsafe {
            memcpy(addr, pdata, data.len());
        }
        print!("Done.");
    }
    Ok(())
}
