// build.rs — compile native bridges and link required libraries.
//
// Bridge strategy:
//   libccd_bridge.c     — wraps libccd (ccd_t, ccdMPRPenetration). Pure C.
//   user_vfs_bridge.cc  — source-overlay of user_vfs.cc; compiles BufferProvider
//                         in this TU, then exposes c2rust_* bridge symbols.
//                         Does NOT forward to libmujoco.a.

use std::path::PathBuf;

fn main() {
    // OpenGL for render module
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=framework=OpenGL");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=GL");

    // ------------------------------------------------------------------
    // Locate mujoco build tree
    // ------------------------------------------------------------------
    let mujoco_build = PathBuf::from(
        std::env::var("MUJOCO_BUILD_DIR").unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent().unwrap()
                .join("mujoco").join("build_out")
                .to_string_lossy().into_owned()
        }),
    );
    let mujoco_src     = mujoco_build.parent().unwrap().join("src");
    let mujoco_include = mujoco_build.parent().unwrap().join("include");
    let deps           = mujoco_build.join("_deps");

    // ------------------------------------------------------------------
    // Bridge 1: libccd_bridge.c  (pure C, wraps ccd_t + ccdMPRPenetration)
    // ------------------------------------------------------------------
    cc::Build::new()
        .file("bridges/libccd_bridge.c")
        .include(deps.join("ccd-src/src"))
        .include(deps.join("ccd-build/src"))
        .define("CCD_STATIC_DEFINE", None)
        .define("_FILE_OFFSET_BITS", "64")
        .flag("-ffp-contract=off")
        .flag("-fno-exceptions")
        .compile("c2rust_libccd_bridge");

    println!("cargo:rustc-link-search=native={}", mujoco_build.join("lib").display());
    println!("cargo:rustc-link-lib=static=ccd");

    // ------------------------------------------------------------------
    // Bridge 2: user_vfs_bridge.cc  (source-overlay of user_vfs.cc)
    //
    // This TU #include's user_vfs.cc so BufferProvider and its Mount template
    // are compiled HERE, not pulled from libmujoco.a.  The bridge then exposes
    // c2rust_mj_addFileVFS_bridge / c2rust_mj_addBufferVFS_bridge which call
    // the same-TU local definitions.
    //
    // We must NOT also link libmujoco.a for VFS symbols — that would cause
    // duplicate symbol linker errors.  We link libmujoco_incomplete.a (which
    // has everything except the symbols this bridge provides) or limit linking.
    // ------------------------------------------------------------------

    // Extract all -I and -D flags from the original user_vfs.cc compile command
    // so this TU builds with the exact same environment.
    let mut vfs_build = cc::Build::new();
    vfs_build
        .cpp(true)
        .file("bridges/user_vfs_bridge.cc")
        // All -I from compile_commands.json for user_vfs.cc
        .include(&mujoco_include)
        .include(&mujoco_src)
        .include(deps.join("marchingcubecpp-src"))
        .include(deps.join("tinyobjloader-src"))
        .include(deps.join("miniz-src"))
        .include(deps.join("miniz-build"))
        .include(deps.join("ccd-src/src"))
        .include(deps.join("ccd-build/src"))
        .include(deps.join("lodepng-src"))
        .include(deps.join("qhull-src/src"))
        .include(deps.join("qhull-src/src/libqhull_r"))
        .include(deps.join("tinyxml2-src"))
        // All -D from compile_commands.json for user_vfs.cc
        .define("CCD_STATIC_DEFINE", None)
        .define("EIGEN_MPL2_ONLY", None)
        .define("MC_IMPLEM_ENABLE", None)
        .define("MINIZ_STATIC_DEFINE", None)
        .define("MUJOCO_DLL_EXPORTS", None)
        .define("TINYOBJLOADER_IMPLEMENTATION", None)
        .define("_FILE_OFFSET_BITS", "64")
        .define("_GNU_SOURCE", None)
        .define("mjUSEPLATFORMSIMD", None)
        .define("mujoco_EXPORTS", None)
        .define("NDEBUG", None)
        .flag("-std=c++20")
        .flag("-ffp-contract=off")
        // Suppress warnings from included upstream source
        .flag("-w")
        .compile("c2rust_user_vfs_bridge");

    // Link remaining mujoco symbols (engine, model, etc.) that VFS calls into.
    // We use libmujoco_incomplete.a if it exists (avoids duplicate VFS symbols),
    // otherwise libmujoco.a — both are in build_out/lib/.
    let lib_dir = mujoco_build.join("lib");
    let incomplete = lib_dir.join("libmujoco_incomplete.a");
    if incomplete.exists() {
        println!("cargo:rustc-link-lib=static=mujoco_incomplete");
    } else {
        println!("cargo:rustc-link-lib=static=mujoco");
    }

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
