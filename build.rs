extern crate bindgen;

use std::path::{Path, PathBuf};

fn main() {
    // Link against the GObject and GLib libraries
    println!("cargo:rustc-link-lib=gobject-2.0");
    println!("cargo:rustc-link-lib=glib-2.0");

    // Print the cargo instruction to link the vips library
    println!("cargo:rustc-link-lib=vips");

    if Path::new("./src/bindings.rs").exists() {
        return;
    }

    let bindings = bindgen::Builder::default()
        .header("/usr/include/vips/vips.h") // Adjust this path to your system
        .clang_arg("-I/usr/lib/gcc/x86_64-linux-gnu/13/include")
        .clang_arg("-I/usr/include/glib-2.0")
        .clang_arg("-I/usr/lib/x86_64-linux-gnu/glib-2.0/include")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .generate_comments(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the src/bindings.rs file
    bindings
        .write_to_file(PathBuf::from("./src").join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
