use std::env;

// Fallback if env var not set - so RLS can compile the crate
const ZMQ_HAS_FALLBACK: &'static str = "";

fn main() {
    let zmq_has = match env::var("ZMQ_HAS") {
        Ok(value) => value,
        Err(..) =>  { 
            println!("ZMQ_HAS environment variable not set, using fallback");
            ZMQ_HAS_FALLBACK.to_string()
        }
    };

    println!("rerun-if-env-changed=ZMQ_HAS");

    for has in zmq_has.split(",") {
        println!("cargo:rustc-cfg=ZMQ_HAS_{}=\"1\"", has.to_uppercase());
    }
}