use std::{env, process::Command};

use tauri::{AppHandle, Manager};

use crate::logger::{self, LogLevel};

#[tauri::command]
pub async fn deploy_wiki(
    wiki_name: &str,
    ssh_url: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let data_dir = app_handle.path().app_data_dir().unwrap();
    let base_path = data_dir.join(wiki_name);
    let dist_directory = base_path.join("dist");

    // Check if git is initialized
    if let Err(err) = env::set_current_dir(&dist_directory) {
        let error = format!("Error while initializing repo: {}", err);
        logger::write_log(&base_path, LogLevel::Error, &error);
        return Err(error);
    }

    if !dist_directory.join(".git").try_exists().unwrap_or(false) {
        match Command::new("git").arg("init").output() {
            Ok(_) => {}
            Err(err) => {
                let error = format!("Error while initializing repo: {}", err);
                logger::write_log(&base_path, LogLevel::Error, &error);
                return Err(error);
            }
        };
    }
    // Run git add
    match Command::new("git").arg("add").arg(".").output() {
        Ok(_) => {}
        Err(err) => {
            let error = format!("Error while adding to repo: {}", err);
            logger::write_log(&base_path, LogLevel::Error, &error);
            return Err(error);
        }
    };
    // Run git commit
    match Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("Wiki Updates")
        .output()
    {
        Ok(_) => {}
        Err(err) => {
            let error = format!("Error while adding to repo: {}", err);
            logger::write_log(&base_path, LogLevel::Error, &error);
            return Err(error);
        }
    };

    let output = match Command::new("git").arg("remote").arg("-v").output() {
        Ok(output) => output,
        Err(err) => {
            let error = format!("Unable to check remote: {}", err);
            logger::write_log(&base_path, LogLevel::Error, &error);
            return Err(error);
        }
    };

    let remote = match String::from_utf8(output.stdout) {
        Ok(remote) => remote,
        Err(err) => {
            let error = format!("Error while reading remote: {}", err);
            logger::write_log(&base_path, LogLevel::Error, &error);
            return Err(error);
        }
    };

    if remote == "".to_string() {
        match Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(format!("{}", ssh_url))
            .output()
        {
            Ok(_) => {}
            Err(err) => {
                let error = format!("Error while adding to repo: {}", err);
                logger::write_log(&base_path, LogLevel::Error, &error);
                return Err(error);
            }
        };
    }

    // TODO: Implement git push automation. Git ssh doesn't allow piping ssh key password into stdin
    // Will need to use ssh-agent to accomplish this in the future.

    // let mut git_command = Command::new("git");
    // git_command.arg("push");
    // git_command.arg("-u");
    // git_command.arg("origin");
    // git_command.arg("main");
    // git_command.stdin(Stdio::piped());

    // let mut proc_handle = git_command.spawn().unwrap();

    // let mut proc_handle_stdin = proc_handle.stdin.take().unwrap();

    // _ = proc_handle_stdin
    //     .write_all("Akstar4321".as_bytes())
    //     .expect("Failed to write to stdin");

    Ok("Wiki Deployed".to_string())
}
