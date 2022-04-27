extern crate kernel32;
extern crate winapi;

use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut stdout = getRawStdout();
    write!(stdout, "Hello, world!")?;
    stdout.write_all(b"Hello, world!")?;
    Ok(())
}

#[cfg(unix)]
fn getRawStdout() -> File {
    use std::os::unix::io::FromRawFd;
    unsafe { File::from_raw_fd(1) }
}

#[cfg(windows)]
fn getRawStdout() -> File {
    use std::os::windows::io::FromRawHandle;
    let h = kernel32::GetStdHandle(winapi::winbase::STD_OUTPUT_HANDLE);
    File::from_raw_handle(h);
}
