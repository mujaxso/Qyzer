//! Build script for zaroxi-lang-syntax
//!
//! This build script is intentionally minimal to avoid circular dependencies.
//! Grammar installation is handled at runtime by the application.

fn main() {
    // Always rerun if build script changes
    println!("cargo:rerun-if-changed=build.rs");

    // Grammar installation is done at runtime by the application.
    // The build script does not install grammars to avoid circular dependencies.
    println!("cargo:warning=Grammar installation is deferred to runtime.");
}
