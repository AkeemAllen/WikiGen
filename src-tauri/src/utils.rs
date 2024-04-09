// Unsure if this is the best way to handle/organize "helper" functions
// in rust. It might not be idiomatic, but it's a start.
use std::env;
use std::{fs, io, path::Path};

/// Copy files from source to destination recursively.
pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn get_os_specific_path(path: String) -> String {
    if env::consts::OS == "windows" {
        return path.replace("/", "\\");
    }
    return path;
}
