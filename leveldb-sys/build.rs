extern crate pkg_config;

use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=LEVELDB_LIB_DIR");
    println!("cargo:rerun-if-env-changed=LEVELDB_STATIC");
    if let Ok(lib_dir) = env::var("LEVELDB_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir);

        let mode = match env::var_os("LEVELDB_STATIC") {
            Some(_) => "static",
            None => "dylib",
        };
        println!("cargo:rustc-link-lib={0}=leveldb", mode);
    } else {
        pkg_config::find_library("libleveldb").unwrap();
    }
}
