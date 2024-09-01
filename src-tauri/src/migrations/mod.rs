pub mod db_migrations;
pub mod file_migrations;

pub async fn run_migrations(app_handle: &tauri::AppHandle) -> Result<(), String> {
    match db_migrations::run_db_migrations(app_handle).await {
        Ok(_) => {}
        Err(err) => {
            println!("Error running database migrations: {}", err);
        }
    }
    match file_migrations::run_file_migrations(app_handle) {
        Ok(_) => {}
        Err(err) => {
            println!("Error running file migrations: {}", err);
        }
    }
    Ok(())
}
