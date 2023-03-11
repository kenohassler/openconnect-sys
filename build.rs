use std::env;
use std::path::PathBuf;

fn main() {
    // Check for required libraries
    let lib = pkg_config::Config::new().probe("openconnect").unwrap();
    let include_args = lib
        .include_paths
        .iter()
        .map(|path| "-I".to_owned() + &path.to_string_lossy());

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Add include paths detected by pkg-config; also allow comments.
        .clang_args(include_args)
        .clang_arg("-fparse-all-comments")
        // Only create bindings for openconnect...
        .allowlist_function(r#"(\w*openconnect_\w*)"#)
        .allowlist_type(r#"(\w*oc_\w*)"#)
        .allowlist_var(r#"(\w*OC_\w*)"#)
        .allowlist_var(r#"(\w*PRG_\w*)"#)
        // ... and exclude some libc stuff manually.
        .blocklist_type(r#"__\w*"#)
        .blocklist_type("uid_t")
        .blocklist_type("time.t")
        .blocklist_type("addrinfo")
        // Put constants from C enums into their own modules.
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
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
