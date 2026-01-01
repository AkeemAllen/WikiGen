use std::{env, process::Command};

use tauri::{AppHandle, Manager};

use crate::logger::{self, LogLevel};

#[tauri::command]
pub async fn commit_wiki_changes(
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

    match Command::new("mkdocs").arg("build").output() {
        Ok(_) => {}
        Err(err) => {
            let error = format!("Error while building wiki: {}", err);
            logger::write_log(&base_path, LogLevel::Error, &error);
            return Err(error);
        }
    };

    let site_directory = dist_directory.join("site");

    if !site_directory.join(".git").try_exists().unwrap_or(false) {
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

    Ok("Wiki Changes Commited and Ready for Deployment".to_string())
}
