extern crate kernel32;
extern crate winapi;

use std::fmt::Debug;
use std::fs::File;
use std::io::{self, Write};
use std::marker::PhantomData;
use std::time::Duration;
use std::{thread, time};

const LOOP_SIZE: u32 = 1000000;

fn main() -> io::Result<()> {
    let mut stdout = createRawStdout();

    let mut testSuite = PerfSuite::new(vec![
        PerfTestData::new::<File>("println!()", |n, _: &mut File| {
            println!("Hello, world! {}\n", n);
        }),
        PerfTestData::new::<File>("print!()", |n, _: &mut File| {
            print!("Hello, world! {}\n", n);
        }),
        PerfTestData::new::<File>("write!()", |n, stdout: &mut File| {
            write!(stdout, "Hello, world! {}\n", n);
        }),
        PerfTestData::new::<File>("stdout.write_all()", |n, stdout: &mut File| {
            stdout.write_all(format!("Hello, world! {}\n", n).as_bytes());
        }),
        PerfTestData::new::<File>("stdout.write()", |n, stdout: &mut File| {
            stdout.write(format!("Hello, world! {}\n", n).as_bytes());
        }),
    ]);

    testSuite.exec(&mut stdout);

    Ok(())
}

struct PerfTestData<A, F>
where
    F: FnMut(u32, &mut A) -> (),
{
    name: String,
    testFn: F,
    argsType: PhantomData<A>,
}

impl<A, F> PerfTestData<A, F>
where
    F: FnMut(u32, &mut A) -> (),
{
    fn new<A_>(name: &str, testFn: F) -> Box<PerfTestData<A_, F>>
    where
        F: FnMut(u32, &mut A_) -> (),
    {
        Box::new(PerfTestData {
            name: String::from(name),
            testFn: testFn,
            argsType: PhantomData,
        })
    }
}

impl<A, F> PerfTest<A> for PerfTestData<A, F>
where
    F: FnMut(u32, &mut A) -> (),
{
    fn runTest(&mut self, n: u32, args: &mut A) -> () {
        (self.testFn)(n, args)
    }

    fn getName(&self) -> &str {
        self.name.as_str()
    }
}

impl<A, F> Debug for PerfTestData<A, F>
where
    F: FnMut(u32, &mut A) -> (),
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("PerfTest {}", self.name))
    }
}

trait PerfTest<A>: Debug {
    fn getName(&self) -> &str;
    fn runTest(&mut self, n: u32, args: &mut A) -> ();
}

#[derive(Debug)]
struct PerfSuite<'a, A> {
    tests: Vec<Box<dyn PerfTest<A> + 'a>>,
}

impl<'a, A> PerfSuite<'a, A> {
    fn new(tests: Vec<Box<dyn PerfTest<A> + 'a>>) -> PerfSuite<'a, A> {
        PerfSuite { tests }
    }

    fn exec(&mut self, args: &mut A) {
        let mut elapsedTimes = Vec::new();

        for test in self.tests.iter_mut() {
            println!("Testing {}...", test.getName());
            // waitSecs(1);

            let elapsedTime = perfTest(test.as_mut(), args);
            elapsedTimes.push(elapsedTime);

            println!("Done!");
            // waitSecs(5);
        }

        for (idx, elapsedTime) in elapsedTimes.iter().enumerate() {
            let test = self.tests.get(idx).unwrap();

            println!(
                "Test {} completed in {} ms",
                test.getName(),
                elapsedTime.as_millis()
            );
        }
    }
}

fn perfTest<A>(test: &mut dyn PerfTest<A>, args: &mut A) -> Duration {
    let timeInstant = time::Instant::now();
    for n in 0..LOOP_SIZE {
        test.runTest(n, args);
    }
    timeInstant.elapsed()
}

fn waitSecs(secs: u64) {
    println!("\n\nWaiting {} second(s)...", secs);
    thread::sleep(time::Duration::from_secs(secs));
}

#[cfg(unix)]
fn createRawStdout() -> File {
    use std::os::unix::io::FromRawFd;
    unsafe { File::from_raw_fd(1) }
}

#[cfg(windows)]
fn createRawStdout() -> File {
    use std::os::windows::io::FromRawHandle;
    let h = kernel32::GetStdHandle(winapi::winbase::STD_OUTPUT_HANDLE);
    File::from_raw_handle(h);
}
