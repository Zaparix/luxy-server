extern crate cc;

use std::env;

fn main() {
    // Get the controller name
    let controller_name = match env::var("CONTROLLER") {
        Ok(val) => val,
        Err(_e) => String::from("dummy"),
    };

    // Build the native controller code
    cc::Build::new()
        .cpp(true)
        .file(format!("src/controller/{}_controller.cpp", controller_name))
        .compile("controller");

    // Execute custom build steps for different controllers
    match controller_name.as_ref() {
        "rpi"  => setup_rpi_controller(),
        "mili" => setup_mili_controller(),
        _      => {}
    }
}

fn setup_rpi_controller() {
    // Include RF24 library
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=dylib=rf24-bcm");
}

fn setup_mili_controller() {
    // Include Mili library
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=dylib=mili");
}
