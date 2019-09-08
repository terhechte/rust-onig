// build.rs

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let in_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

   // std::fs::write("/tmp/uxy.txt", &format!("{}", out_dir)).expect("Unable to write file");
   std::fs::copy(&format!("{}/libonig.a", &in_dir), 
                 &format!("{}/libonig.a", &out_dir));

    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=onig");
}
