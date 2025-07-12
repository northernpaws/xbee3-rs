use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if !target.ends_with("eabi") && !target.ends_with("eabihf") {
        panic!("attempting to build for target `{}`, but expected thumbv7em-none-eabi[hf]", target);
    }

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}