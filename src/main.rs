#![allow(non_snake_case)]
#![warn(non_camel_case_types)]

extern crate kernel32;
extern crate winapi;

mod dynlib;
mod perf_test;

fn main() {
    // perf_test::runTests();
    dynlib::runDynlib();
}
