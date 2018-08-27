use std::env;
use std::path::Path;

// Fallbacks if env vars not set - so RLS can compile the crate
const LIBZMQ_PREFIX_FALLBACK: &'static str = "/usr/lib/arm-linux-gnueabihf";
const INCZMQ_PREFIX_FALLBACK: &'static str = "/usr/include";

fn lib_prefix_dir(env_name: &str, dir: &str) -> String {
    match env::var(env_name) {
        Ok(value) => value,
        Err(..) => match env::var("LIBZMQ_PREFIX") {
            Ok(prefix) => Path::new(&prefix).join(dir).to_string_lossy().into_owned(),
            Err(..) => { 
                println!("Neither {} nor LIBZMQ_PREFIX environment variables set, using fallback", env_name);
                LIBZMQ_PREFIX_FALLBACK.to_string()
            }
        }
    }
}
fn inc_prefix_dir(env_name: &str) -> String {
    match env::var(env_name) {
        Ok(value) => value,
        Err(..) => {
            println!("LIBZMQ_INCLUDE_DIR environment variable not set, using fallback");
            INCZMQ_PREFIX_FALLBACK.to_string()
        }
    }
}

fn main() {
    let lib_path = lib_prefix_dir("LIBZMQ_LIB_DIR", "lib");
    let include = inc_prefix_dir("LIBZMQ_INCLUDE_DIR");

    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:include={}", include);
    println!("cargo:rustc-link-lib=zmq");
}
