// Unsure if this is the best way to handle/organize "helper" functions
// in rust. It might not be idiomatic, but it's a start.
use serde::Serialize;
use std::process::{Command, Stdio};
use std::{fs, io, path::Path};
use sysinfo::{Pid, System};

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

#[derive(Debug, Serialize, Clone)]
enum MkdocsServerStatus {
    Started,
    Running,
    Stopped,
}

#[derive(Debug, Serialize, Clone)]
pub struct Payload {
    message: String,
    status: MkdocsServerStatus,
    process_id: usize,
}

// User will need to have python3 or mkdocs installed.
// Either inform the user to install it or install it for them.
#[tauri::command]
pub async fn spawn_mkdocs_process(
    mkdocs_file_path: String,
    wiki_server_address: String,
) -> Result<Payload, String> {
    let mut mkdocs_command = Command::new("mkdocs");
    let mkdocs_serve = mkdocs_command
        .arg("serve")
        .arg("-a")
        .arg(&wiki_server_address)
        .arg("-f")
        .arg(Path::new(&mkdocs_file_path));

    let child_stdout = mkdocs_serve.stdout(Stdio::piped()).spawn().unwrap();

    Ok(Payload {
        message: format!("Mkdocs Server started at {}", &wiki_server_address),
        status: MkdocsServerStatus::Started,
        process_id: child_stdout.id() as usize,
    })
}

#[tauri::command]
pub async fn kill_mkdocs_process(process_id: usize) -> Result<Payload, String> {
    let system = System::new_all();
    if let Some(process) = system.process(Pid::from(process_id)) {
        process.kill();
    }

    Ok(Payload {
        message: format!("Killed Mkdocs Server"),
        status: MkdocsServerStatus::Stopped,
        process_id: 0,
    })
}

#[tauri::command]
pub async fn check_process_status(process_id: usize) -> Result<Payload, String> {
    let system = System::new_all();
    if system.process(Pid::from(process_id)).is_some() {
        Ok(Payload {
            message: format!("Process {} is running", process_id),
            status: MkdocsServerStatus::Running,
            process_id: process_id,
        })
    } else {
        Ok(Payload {
            message: format!("Process {} is not running", process_id),
            status: MkdocsServerStatus::Stopped,
            process_id: 0,
        })
    }
}
