use std::env::consts::{OS, ARCH};
use chrono::prelude::Local;

#[cfg(debug_assertions)]
const BUILD_TYPE: &'static str = "debug";
#[cfg(not(debug_assertions))]
const BUILD_TYPE: &'static str = "release";

fn main() {
    let version_string =
        format!("{} {} ({} build, {} [{}], {})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            BUILD_TYPE,
            OS, ARCH,
            Local::now().format("%d %b %Y, %T"));

    println!("cargo:rustc-env=VERSION_STRING={}", version_string);
}