use std::fs::File;
use std::io::{self, Write};
use std::os::unix::io::FromRawFd;

fn main() -> io::Result<()> {
    let mut stdout = unsafe { File::from_raw_fd(1) };
    stdout.write_all(b"Hello, world!")?;
    Ok(())

    // println!("Hello, world!");
    // io::stdout().write(b"Hello, world!")?;
}
