use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let vendor_dir = manifest_dir.join("vendor");

    // Tell cargo to rerun if the vendored header changes
    println!("cargo:rerun-if-changed=vendor/zimtohrli.h");
    println!("cargo:rerun-if-changed=vendor/ZIMTOHRLI_VERSION");
    println!("cargo:rerun-if-changed=src/bridge.h");
    println!("cargo:rerun-if-changed=src/bridge.cc");

    let mut build = cxx_build::bridge("src/lib.rs");
    build
        .file("src/bridge.cc")
        .include(&vendor_dir)
        .include(manifest_dir.join("src"))
        .flag_if_supported("-std=c++17")
        .flag_if_supported("/std:c++17"); // MSVC

    if env::var_os("CARGO_FEATURE_FAST_MATH").is_some() {
        build
            // Match upstream CMake flags for non-MSVC compilers.
            .flag_if_supported("-fassociative-math")
            .flag_if_supported("-freciprocal-math")
            .flag_if_supported("-fno-signed-zeros")
            .flag_if_supported("-fno-math-errno")
            .flag_if_supported("/fp:fast"); // MSVC
    }

    build.compile("zimtohrli-sys");
}
