use std::env;

pub fn configure() {
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-env-changed=PROFILE");

    let maybe_libsodium = if cfg!(feature = "libsodium") {
        let lib_dir = env::var("DEP_SODIUM_LIB")
            .expect("build metadata `DEP_SODIUM_LIB` required");
        let include_dir = env::var("DEP_SODIUM_INCLUDE")
            .expect("build metadata `DEP_SODIUM_INCLUDE` required");

        Some(zeromq_src::LibLocation::new(lib_dir, include_dir))
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
