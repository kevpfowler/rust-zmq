use std::env;
use std::path::Path;

fn prefix_dir(env_name: &str, dir: &str) -> String {
    match env::var(env_name) {
        Ok(value) => value,
        Err(..) => match env::var("LIBZMQ_PREFIX") {
            Ok(prefix) => Path::new(&prefix).join(dir).to_string_lossy().into_owned(),
            Err(..) => panic!("Please set the {} or LIBZMQ_PREFIX environment variable", env_name)
        }
    }
}

fn main() {
    let lib_path = prefix_dir("LIBZMQ_LIB_DIR", "lib");
    let include = prefix_dir("LIBZMQ_INCLUDE_DIR", "include");

    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:include={}", include);
    println!("cargo:rustc-link-lib=zmq");
}
