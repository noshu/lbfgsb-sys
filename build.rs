// build.rs

use std::process::Command;
use std::env::{var};
use std::path::PathBuf;

macro_rules! feature(
    ($name:expr) => (var(concat!("CARGO_FEATURE_", $name)).is_ok());
);

macro_rules! variable(
    ($name:expr) => (var($name).unwrap());
);

fn main() {
    let kind = if feature!("STATIC") { "static" } else { "dylib" };
    let source = PathBuf::from("fortran");
    let output = PathBuf::from(variable!("OUT_DIR").replace(r"\", "/"));
    let os = if cfg!(target_os = "macos"){"Macos"}
             else if cfg!(target_os = "windows"){"Windows"}
             else {"Linux"};
    let com = if os=="Windows" {"mingw32-make"}else{"make"};
    run(Command::new(com)
                .arg(kind)
                .arg(format!("OUTPUT={}",output.display()))
                .arg(format!("OSNAME={}",os))
                .current_dir(&source));

    println!("cargo:rustc-link-search={}", output.display());
    println!("cargo:rustc-link-lib={}=lbfgs",kind);
    println!("cargo:rustc-link-lib=dylib=gfortran");
}
fn run(command: &mut Command) {
    println!("Running: {:?}", command);
    match command.status() {
        Ok(status) => if !status.success() {
            panic!("`{:?}` failed: {}", command, status);
        },
        Err(error) => {
            panic!("failed to execute `{:?}`: {}", command, error);
        },
    }
}
