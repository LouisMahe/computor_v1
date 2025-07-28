use std::process::Command;

fn main() {
    let status = Command::new("make")
        .current_dir("polylib")
        .status()
        .expect("Failed to build C lib");

    if !status.success() {
        panic!("Could not link with polylib.a");
    }

    println!("cargo:rustc-link-search=native=polylib");
    println!("cargo:rustc-link-lib=static=polylib");
}