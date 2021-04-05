// Build script
// Copy resources folder
fn main() {
    println!("cargo:rerun-if-changed=resources");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    let src = PathBuf::from("./resources");
    let dst = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("resources");
    let _ = copy_dir_all(src, dst);
}

use std::path::{Path, PathBuf};
use std::{fs, io};

// See: https://stackoverflow.com/a/65192210/12347616
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
