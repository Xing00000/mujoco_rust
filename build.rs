// build.rs — compile native bridges and link required libraries.
//
// Bridges:
//   bridges/libccd_bridge.c  — wraps libccd (ccd_t, ccdMPRPenetration)
//   bridges/user_vfs_bridge.cc — wraps mj_addFileVFS/BufferVFS via libmujoco

use std::path::PathBuf;

fn main() {
    // OpenGL for render module
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=framework=OpenGL");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=GL");

    let mujoco_build = PathBuf::from(
        std::env::var("MUJOCO_BUILD_DIR")
            .unwrap_or_else(|_| {
                // Default to the build_out dir relative to this crate
                let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                // mujoco_rust/  →  mujoco_rust/../mujoco/build_out
                manifest
                    .parent()
                    .unwrap()
                    .join("mujoco")
                    .join("build_out")
                    .to_string_lossy()
                    .into_owned()
            }),
    );

    let mujoco_src = mujoco_build.parent().unwrap().join("src");
    let mujoco_include = mujoco_build.parent().unwrap().join("include");

    // ------------------------------------------------------------------
    // Bridge 1: libccd_bridge.c
    // ------------------------------------------------------------------
    let ccd_src_include = mujoco_build.join("_deps/ccd-src/src");
    let ccd_build_include = mujoco_build.join("_deps/ccd-build/src");

    cc::Build::new()
        .file("bridges/libccd_bridge.c")
        .include(&ccd_src_include)
        .include(&ccd_build_include)
        .define("CCD_STATIC_DEFINE", None)
        .define("_FILE_OFFSET_BITS", "64")
        .flag("-ffp-contract=off")
        .flag("-fno-exceptions")
        .compile("c2rust_libccd_bridge");

    // Link against the pre-built libccd
    println!(
        "cargo:rustc-link-search=native={}",
        mujoco_build.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=ccd");

    // ------------------------------------------------------------------
    // Bridge 2: user_vfs_bridge.cc (C++)
    // ------------------------------------------------------------------
    cc::Build::new()
        .cpp(true)
        .file("bridges/user_vfs_bridge.cc")
        .include(&mujoco_include)
        .include(&mujoco_src)
        .include(mujoco_build.join("_deps/ccd-src/src"))
        .define("CCD_STATIC_DEFINE", None)
        .define("_FILE_OFFSET_BITS", "64")
        .define("NDEBUG", None)
        .flag("-std=c++20")
        .flag("-ffp-contract=off")
        .compile("c2rust_user_vfs_bridge");

    // Link libmujoco (contains BufferProvider, VFS, mj_addFileVFS, etc.)
    println!("cargo:rustc-link-lib=static=mujoco");

    // C++ runtime
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=stdc++");

    // Rerun triggers
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=bridges/libccd_bridge.c");
    println!("cargo:rerun-if-changed=bridges/libccd_bridge.h");
    println!("cargo:rerun-if-changed=bridges/user_vfs_bridge.cc");
}
