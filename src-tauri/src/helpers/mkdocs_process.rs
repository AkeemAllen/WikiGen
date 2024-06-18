// Unsure if this is the best way to handle/organize "helper" functions
// in rust. It might not be idiomatic, but it's a start.
use serde::Serialize;
use std::net::TcpStream;
use std::path::Path;
use std::process::{id, Command};
use sysinfo::{Pid, System};
use tauri::AppHandle;

#[derive(Debug, Serialize, Clone)]
enum MkdocsServerStatus {
    Started,
    Running,
    Stopped,
    Occupied,
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
    wiki_name: &str,
    app_handle: AppHandle,
    port: u16,
) -> Result<Payload, Payload> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");

    let mut is_port_in_use = false;

    match TcpStream::connect(("0.0.0.0", port)) {
        Ok(_) => is_port_in_use = true,
        Err(_) => {}
    }

    if is_port_in_use {
        let system = System::new_all();
        let mut killed_process = false;
        for (_, process) in system.processes() {
            if process.parent() == Some(Pid::from_u32(id())) {
                killed_process = true;
                process.kill();
                break;
            }
        }

        if !killed_process {
            return Err(Payload {
                message: format!("Port {} is already in use", &port),
                status: MkdocsServerStatus::Occupied,
                process_id: 0,
            });
        }
    }

    let mut mkdocs_command = Command::new("mkdocs");
    let mkdocs_serve = mkdocs_command
        .arg("serve")
        .arg("-a")
        .arg(format!("0.0.0.0:{}", &port))
        .arg("-f")
        .arg(mkdocs_yaml_file_path);

    let child_stdout = mkdocs_serve.spawn().expect("Failed to start Mkdocs Server");

    Ok(Payload {
        message: format!("Mkdocs Server started at localhost:{}", &port),
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
