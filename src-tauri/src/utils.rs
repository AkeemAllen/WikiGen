// Unsure if this is the best way to handle/organize "helper" functions
// in rust. It might not be idiomatic, but it's a start.
use std::process::{Command, Stdio};
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

// User will need to have python3 or mkdocs installed.
// Either inform the user to install it or install it for them.
#[tauri::command]
pub fn spawn_mkdocs_process(mkdocs_file_path: &str) {
    let path = Path::new(mkdocs_file_path);
    // println!("{}", mkdocs_file_path.to_string());
    let command = Command::new("mkdocs")
        .arg("serve")
        .arg("-f")
        .arg(path)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    command.stdout.unwrap();
}
