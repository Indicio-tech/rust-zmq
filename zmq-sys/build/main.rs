use std::env;
use std::path::PathBuf;

fn get_lib_dir() -> zeromq_src::LibLocation {
    let target = env::var("TARGET").expect("TARGET not set");

    if target.contains("windows-gnu") {
        // MinGW
        let base = env::var("CARGO_MANIFEST_DIR").unwrap();
        let lib = PathBuf::from(&base).join("libsodium/mingw/libsodium-win64/lib");
        let include = PathBuf::from(&base).join("libsodium/mingw/libsodium-win64/include");
        zeromq_src::LibLocation::new(lib, include)
    } else if target.contains("windows-msvc") {
        // MSVC
        let base = env::var("CARGO_MANIFEST_DIR").unwrap();
        let lib = PathBuf::from(&base).join("libsodium/msvc/x64/Release/v143/static");
        let include = PathBuf::from(&base).join("libsodium/msvc/include");
        zeromq_src::LibLocation::new(lib, include)
    } else {
        // Unix-like (macOS, Linux, etc.)
        let lib = env::var("DEP_SODIUM_LIB")
            .expect("build metadata `DEP_SODIUM_LIB` required");
        let include = env::var("DEP_SODIUM_INCLUDE")
            .expect("build metadata `DEP_SODIUM_INCLUDE` required");
        zeromq_src::LibLocation::new(lib, include)
    }
}

pub fn configure() {
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-env-changed=PROFILE");
    
    // Note that by default `libzmq` builds without `libsodium` by instead
    // relying on `tweetnacl`. However since this `tweetnacl` [has never been
    // audited nor is ready for production](https://github.com/zeromq/libzmq/issues/3006),
    // we link against `libsodium` to enable `ZMQ_CURVE`.
    let maybe_libsodium = if cfg!(feature = "libsodium") {
            Some(get_lib_dir())
    } else {
        None
    };

    zeromq_src::Build::new()
        .with_libsodium(maybe_libsodium)
        .build();
   
}

fn main() {
    configure()
}
