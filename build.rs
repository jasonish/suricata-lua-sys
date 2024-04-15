use std::{env, path::PathBuf, process::Command};

fn main() {
    let host = env::var_os("HOST")
        .map(|host| host.into_string().unwrap())
        .unwrap();

    let out_dir = env::var_os("OUT_DIR").map(PathBuf::from).unwrap();
    let build_dir = out_dir.join("lua");

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap();

    // Copy the source into a build directory as we shouldn't build in the
    // source directory.
    let _ = std::fs::remove_dir_all(&build_dir);
    std::fs::create_dir_all(&out_dir).unwrap();
    let opts = fs_extra::dir::CopyOptions {
        copy_inside: true,
        ..Default::default()
    };
    fs_extra::dir::copy(manifest_dir.join("lua"), &build_dir, &opts).unwrap();

    let mut command = Command::new("make");
    if host.contains("windows") {
        command.arg("mingw");
    }

    // Required for building into a shared library.
    command.arg("MYCFLAGS=-fPIC");

    // Don't inherit parent MAKEFLAGS, they may not be suitable for
    // this build.
    command.env_remove("MAKEFLAGS");

    let status = command.current_dir(&build_dir).status().unwrap();
    if !status.success() {
        panic!("build failed");
    }

    println!("cargo:rustc-link-lib=static=lua");
    println!("cargo:rustc-link-search=native={}", build_dir.display());
}
