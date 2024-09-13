use std::path::PathBuf;

pub mod db_migrations;
pub mod file_migrations;

pub async fn run_migrations(base_path: &PathBuf, resources_path: &PathBuf) -> Result<(), String> {
    match db_migrations::run_db_migrations(base_path, resources_path).await {
        Ok(_) => {}
        Err(err) => {
            println!("Error running database migrations: {}", err);
        }
    }
    match file_migrations::run_file_migrations(base_path, resources_path) {
        Ok(_) => {}
        Err(err) => {
            println!("Error running file migrations: {}", err);
        }
    }
    Ok(())
}
