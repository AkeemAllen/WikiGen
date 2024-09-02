#[cfg(test)]
pub mod tests {
    use crate::migrations::run_migrations;

    #[test]
    fn test_run_migrations() {
        let base_path = std::path::PathBuf::from(
            "/Users/akeemallen/Library/Application Support/com.wikigen.dev",
        );
        match tauri::async_runtime::block_on(run_migrations(&base_path)) {
            Ok(_) => {}
            Err(err) => {
                panic!("Failed to run migrations: {}", err);
            }
        }
    }
}
