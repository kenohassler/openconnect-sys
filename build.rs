use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // Check for required libraries
    // let pkg = pkg_config::Config::new()
    //     .probe("openconnect")
    //     .expect("libopenconnect needs to be installed in order to build this crate");
    // println!("{pkg:?}");

    // Tell cargo to look for the openconnect library in `lib_path`
    // println!("cargo:rustc-link-search={}", );
    println!("cargo:rustc-link-lib=openconnect");

    // Tell cargo to invalidate the built crate whenever the header changes
    let header = Path::new("src/wrapper.h");
    println!("cargo:rerun-if-changed={}", header.display());

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header(header.to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Only create bindings for openconnect...
        .allowlist_type(r#"(\w*oc_\w*)"#)
        .allowlist_function(r#"(\w*openconnect_\w*)"#)
        // ... and exclude some libc stuff manually.
        .blocklist_type(r#"__\w*"#)
        .blocklist_type("uid_t")
        .blocklist_type("time.t")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
