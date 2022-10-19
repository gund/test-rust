#![allow(non_snake_case)]
#![warn(non_camel_case_types)]

extern crate kernel32;
extern crate winapi;

use std::cell::RefCell;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::marker::PhantomData;
use std::{thread, time};

pub fn runTests() {
    let mut stdout = createRawStdout();
    let stdoutRef = RefCell::new(stdout);

    let mut testSuite = PerfSuite::new(
        vec![
            PerfTestData::new::<()>("println!()", |n, _| {
                println!("Hello, world! {}\n", n);
            }),
            PerfTestData::new::<()>("print!()", |n, _| {
                print!("Hello, world! {}\n", n);
            }),
            PerfTestData::new::<()>("write!()", |n, _| {
                write!(stdoutRef.borrow_mut(), "Hello, world! {}\n", n);
            }),
            PerfTestData::new::<()>("stdout.write_all()", |n, _| {
                stdoutRef
                    .borrow_mut()
                    .write_all(format!("Hello, world! {}\n", n).as_bytes());
            }),
            PerfTestData::new::<()>("stdout.write()", |n, _| {
                stdoutRef
                    .borrow_mut()
                    .write(format!("Hello, world! {}\n", n).as_bytes());
            }),
        ],
        1000000,
    );

    testSuite.exec(&mut ());
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
    loopSize: u32,
}

struct PerfSuiteResults {
    totalTests: u32,
    executedTests: u32,
    failedTests: u32,
    details: Vec<PerfSuiteResultDetails>,
}

struct PerfSuiteResultDetails {
    testName: String,
    duration: time::Duration,
    executedTimes: u32,
    failedTimes: u32,
    failureReason: String,
}

impl<'a, A> PerfSuite<'a, A> {
    fn new(tests: Vec<Box<dyn PerfTest<A> + 'a>>, loopSize: u32) -> PerfSuite<'a, A> {
        PerfSuite { tests, loopSize }
    }

    fn exec(&mut self, args: &mut A) {
        let mut elapsedTimes = Vec::new();

        for test in self.tests.iter_mut() {
            println!("Testing {}...", test.getName());
            PerfSuite::<A>::waitSecs(1);

            let elapsedTime = perfTest(test.as_mut(), args, self.loopSize);
            elapsedTimes.push(elapsedTime);

            println!("Done!");
            PerfSuite::<A>::waitSecs(1);
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

    fn waitSecs(secs: u64) {
        println!("\n\nWaiting {} second(s)...", secs);
        thread::sleep(time::Duration::from_secs(secs));
    }
}

fn perfTest<A>(test: &mut dyn PerfTest<A>, args: &mut A, loopSize: u32) -> time::Duration {
    let timeInstant = time::Instant::now();

    for n in 0..loopSize {
        test.runTest(n, args);
    }

    timeInstant.elapsed()
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
