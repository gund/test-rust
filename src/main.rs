extern crate kernel32;
extern crate winapi;

mod dynlib;
mod perf_test;

fn main() {
    // perf_test::runTests();
    dynlib::runDlib();
}
