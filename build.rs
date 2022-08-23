use autotools::Config;
use bindgen;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let src_path = extract_tarball("openconnect-9.01").expect("Extracting source tarball failed");

    // check for required libraries
    let pkg = pkg_config::Config::new();
    pkg.probe("zlib").unwrap();
    pkg.probe("openssl").unwrap();
    pkg.probe("libxml-2.0").unwrap();

    // build using autotools
    let dst = Config::new(src_path)
        .with("vpnc-script", Some("/etc/vpnc/vpnc-script"))
        .disable("nls", None)
        .build();

    let lib_path = dst.join("lib");
    let include_path = dst.join("include");
    debug_assert!(lib_path.exists());
    debug_assert!(include_path.exists());

    // Tell cargo to look for the openconnect library in `lib_path`
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=openconnect");

    // Tell cargo to invalidate the built crate whenever the header changes
    let header = include_path.join("openconnect.h");
    println!("cargo:rerun-if-changed={}", header.display());

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header(header.to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // don't create bindings for the standard library...
        .allowlist_type(r#"(\w*oc_\w*)"#)
        .allowlist_function(r#"(\w*openconnect_\w*)"#)
        //.blocklist_type(r#"__\w*"#)
        //.blocklist_type("uid_t")
        //.blocklist_type("time.t")
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

/// Uncompress the tarball in `include/` to a folder in `target/cache/`.
fn extract_tarball(name: &str) -> Result<PathBuf, std::io::Error> {
    use flate2::read::GzDecoder;
    use std::fs::File;
    use tar::Archive;

    let tarball = Path::new("include").join(name.to_owned() + ".tar.gz");
    let dst = Path::new("target/cache");
    Archive::new(GzDecoder::new(File::open(tarball)?)).unpack(dst)?;

    Ok(dst.join(name))
}
