use std::process::Command;

fn main() {
    let status = Command::new("make")
        .status()
        .expect("Failed to execute make");

    if !status.success() {
        panic!("Makefile execution failed!");
    }
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=getcwd");
}