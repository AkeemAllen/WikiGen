// Unsure if this is the best way to handle/organize "helper" functions
// in rust. It might not be idiomatic, but it's a start.
use serde::Serialize;
use std::env;
use std::net::TcpStream;
use std::path::Path;
use std::process::{id, Command, Stdio};
use sysinfo::{Pid, System};
use tauri::AppHandle;

use crate::logger;

#[derive(Debug, Serialize, Clone)]
enum MkdocsServerStatus {
    Started,
    Running,
    Stopped,
    Occupied,
    Error,
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
    port: u16,
    app_handle: AppHandle,
) -> Result<Payload, Payload> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let dist_directory = base_path.join(wiki_name).join("dist");

    // Check if git is initialized
    if let Err(err) = env::set_current_dir(&dist_directory) {
        let error = format!("Error while switching to dist directory: {}", err);
        logger::write_log(&base_path, logger::LogLevel::Error, &error);
        return Err(Payload {
            message: error,
            status: MkdocsServerStatus::Error,
            process_id: 0,
        });
    }

    let mut is_port_in_use = false;

    match TcpStream::connect(("0.0.0.0", port)) {
        Ok(_) => is_port_in_use = true,
        Err(_) => {}
    }

    if is_port_in_use {
        return Err(Payload {
            message: format!("Port {} is already in use", &port),
            status: MkdocsServerStatus::Occupied,
            process_id: 0,
        });
    }

    let mut mkdocs_command = Command::new("mkdocs");
    let mkdocs_serve = mkdocs_command
        .arg("serve")
        .arg("-a")
        .arg(format!("0.0.0.0:{}", &port))
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null());

    let mkdocs_handle = match mkdocs_serve.spawn() {
        Ok(handle) => handle,
        Err(err) => {
            let message = format!(
                "Failed to start Server: {}, path: {:?}",
                err, &dist_directory
            );
            logger::write_log(&base_path, logger::LogLevel::Error, &message);
            return Err(Payload {
                message,
                status: MkdocsServerStatus::Error,
                process_id: 0,
            });
        }
    };

    Ok(Payload {
        message: format!("Mkdocs Server started at localhost:{}", &port),
        status: MkdocsServerStatus::Started,
        process_id: mkdocs_handle.id() as usize,
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
            process_id,
        })
    } else {
        Ok(Payload {
            message: format!("Process {} is not running", process_id),
            status: MkdocsServerStatus::Stopped,
            process_id: 0,
        })
    }
}
