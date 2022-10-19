#![allow(non_snake_case)]
#![warn(non_camel_case_types)]

#[no_mangle]
pub extern "C" fn lib_test() {
    println!("Hello from the library!");
}

#[no_mangle]
pub extern "C" fn lib_test1(str: &str) {
    println!("Hello from the library param - {}!", str);
}
