use std::{env, path};

pub fn runDlib() {
    let libPath = env::args().nth(1).unwrap();
    let fullPath = normalizePath(
        libPath.as_str(),
        env::current_dir().unwrap().to_str().unwrap(),
    );

    let lib = loadDynLib(fullPath.as_str()).unwrap();

    execDynLib::<()>(&lib, "lib_test");
}

fn normalizePath(pathStr: &str, cwd: &str) -> String {
    let path = path::Path::new(pathStr);
    let mut normalPath = path::PathBuf::new();

    if path.is_relative() {
        normalPath.push(cwd);
        normalPath.push(path);
    } else {
        normalPath.push(path);
    }

    normalPath
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

fn execDynLib<R>(lib: &libloading::Library, method: &str) -> Result<R, libloading::Error> {
    println!("Executing dynlib method {:?}::{}", lib, method);

    unsafe {
        let libMethod: libloading::Symbol<unsafe extern "C" fn() -> R> =
            lib.get(method.as_bytes())?;
        Ok(libMethod())
    }
}

fn loadDynLib(path: &str) -> Result<libloading::Library, libloading::Error> {
    println!("Loading dynlib {}", path);

    unsafe { Ok(libloading::Library::new(path)?) }
}
