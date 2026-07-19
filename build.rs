// Links OpenGL framework for render module extern C function calls.
fn main() {
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=framework=OpenGL");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=GL");
    println!("cargo:rerun-if-changed=build.rs");
}
