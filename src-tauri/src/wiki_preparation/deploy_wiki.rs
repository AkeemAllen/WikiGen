use std::{fs, process::Command};

use sysinfo::Pid;
use tauri::AppHandle;

#[tauri::command]
pub fn deploy_wiki(wiki_name: &str, app_handle: AppHandle) -> Result<String, String> {
    let data_dir = app_handle.path_resolver().app_data_dir().unwrap();
    let base_path = data_dir.join(wiki_name);
    let dist_path = base_path.join("dist");

    let generator_assets = app_handle
        .path_resolver()
        .resource_dir()
        .unwrap()
        .join("resources")
        .join("generator_assets");

    match fs::copy(
        generator_assets.join("requirements.txt"),
        dist_path.join("requirements.txt"),
    ) {
        Ok(_) => {}
        Err(err) => {
            return Err(format!(
                "Failed to copy requirements.txt to dist directory: {}",
                err
            ));
        }
    };

    let initialized = match dist_path.join(".git").try_exists() {
        Ok(result) => result,
        Err(err) => return Err(format!("Unable to check if git exists: {}", err)),
    };

    if !initialized {
        let mut git = Command::new("git");
        let git_init = git.arg("init").arg(&dist_path);
        let mut child_stdout = match git_init.spawn() {
            Ok(result) => result,
            Err(err) => return Err(format!("Failed to initialize wiki repo command: {}", err)),
        };

        match child_stdout.wait() {
            Ok(_) => {}
            Err(err) => return Err(format!("Init not waiting: {}", err)),
        }
    }

    let mut git = Command::new("git");
    let git_add = git.arg("-C").arg(&dist_path).arg("add").arg(".");

    let mut child_stdout = match git_add.spawn() {
        Ok(result) => result,
        Err(err) => return Err(format!("Failed to run git add: {}", err)),
    };

    match child_stdout.wait() {
        Ok(_) => {}
        Err(err) => return Err(format!("Add not waiting: {}", err)),
    }

    let mut git = Command::new("git");
    let git_commit = git
        .arg("-C")
        .arg(&dist_path)
        .arg("commit")
        .arg("-m")
        .arg(format!("Wiki Updated"));

    let mut child_stdout = match git_commit.spawn() {
        Ok(result) => result,
        Err(err) => return Err(format!("Failed to run git add: {}", err)),
    };

    match child_stdout.wait() {
        Ok(_) => {}
        Err(err) => return Err(format!("Add not waiting: {}", err)),
    }

    let mut git = Command::new("git");
    let git_show_origin = git
        .arg("-C")
        .arg(&dist_path)
        .arg("remote")
        .arg("show")
        .arg("origin");

    let mut child_stdout = match git_show_origin.spawn() {
        Ok(result) => {
            println!("Output {:?}", result.stdout);
            println!("Error {:?}", result.stderr);
            result
        }
        Err(err) => return Err(format!("Failed to run git show origin: {}", err)),
    };

    match child_stdout.wait() {
        Ok(result) => {
            print!("Output {:?}", result);
        }
        Err(err) => return Err(format!("Failed to run git show origin: {}", err)),
    }

    Ok("Wiki Deployed".to_string())
}
