use std::env::consts::{OS, ARCH};
use chrono::prelude::Local;
use std::fs;

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
 
    fs::write("makepack.bat",
    format!("strip target\\release\\nkscgeosync.exe\
            \n\"C:\\Program Files\\7-Zip\\7z.exe\" a {0}_{1}_{2}_{3}.zip LICENSE README.md\
            \ncd target\\release\
            \n\"C:\\Program Files\\7-Zip\\7z.exe\" a ..\\..\\{0}_{1}_{2}_{3}.zip nkscgeosync.exe\
            \ncd ..\\..",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            OS, ARCH)).expect("That didn't quite go to plan.");    

}