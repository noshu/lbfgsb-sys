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
    run(Command::new("make")
                .arg(kind)
                .arg(format!("OUTPUT={}", output.display()))
                .arg(format!("OSNAME={}", os))
                .current_dir(&source));

    println!("cargo:rustc-link-search={}", output.display());
    println!("cargo:rustc-link-lib={}=lbfgs", kind);
    println!("cargo:rustc-link-lib=dylib=gcc");

    let target = variable!("TARGET");
    let mut fc_lib_type = "dylib";
    if target == "x86_64-apple-darwin" || target == "x86_64-pc-windows-gnu" {
        fc_lib_type = "static";

        // Poke $FC$ for static lib folder
        let fc_out = Command::new(variable!("FC"))
                              .arg("-print-file-name=libgfortran.a")
                              .output()
                              .expect("Failed to find libgfortran.a");
        let fc_stdout = String::from_utf8_lossy(&fc_out.stdout);
        let fc_lib_cwd = PathBuf::from(fc_stdout.to_string());
        let fc_lib_pwd = fc_lib_cwd.parent().expect("Path to libgfortran.a not found");
        println!("cargo:rustc-link-search={}", fc_lib_pwd.to_str().unwrap());
    }

    println!("cargo:rustc-link-lib={}=gfortran", fc_lib_type);
    println!("cargo:rustc-link-lib={}=quadmath", fc_lib_type);
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
