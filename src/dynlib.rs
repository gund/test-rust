#![allow(non_snake_case)]
#![warn(non_camel_case_types)]

use std::{env, marker::PhantomData, path};

pub fn runDynlib() -> Result<(), libloading::Error> {
    let libPath = env::args()
        .nth(1)
        .expect("Missing <libPath> positional argument!");
    let fullPath = normalizePath(libPath.as_str());

    let lib = DynLib::load(fullPath.as_str())?;
    let libMethod = lib.getMethod::<(), ()>("lib_test")?;
    let libMethod1 = lib.getMethod::<&str, ()>("lib_test1")?;

    libMethod.call(());
    libMethod1.call("hi");

    Ok(())
}

#[derive(Debug)]
struct DynLib {
    lib: libloading::Library,
}

impl DynLib {
    fn load(path: &str) -> Result<DynLib, libloading::Error> {
        println!("Loading lib {}", path);
        let lib = unsafe { libloading::Library::new(path)? };
        Ok(DynLib { lib })
    }

    fn getMethod<A, R>(&self, name: &'static str) -> Result<DynLibMethod<A, R>, libloading::Error> {
        DynLibMethod::<A, R>::new(self, name)
    }
}

#[derive(Debug)]
struct DynLibMethod<'a, A, R> {
    name: &'a str,
    method: libloading::Symbol<'a, unsafe extern "C" fn(A) -> R>,
}

impl<'a, A, R> DynLibMethod<'a, A, R> {
    fn new(lib: &'a DynLib, name: &'a str) -> Result<DynLibMethod<'a, A, R>, libloading::Error> {
        let method = unsafe {
            let libMethod: libloading::Symbol<unsafe extern "C" fn(A) -> R> =
                lib.lib.get(name.as_bytes())?;
            libMethod
        };

        Ok(DynLibMethod::<'a, A, R> { name, method })
    }

    fn call(&self, args: A) -> R {
        println!("Calling method {:?}", self.method);
        unsafe { (self.method)(args) }
    }

    fn getName(&self) -> &str {
        self.name
    }
}

fn normalizePath(pathStr: &str) -> String {
    path::Path::new(pathStr)
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}
