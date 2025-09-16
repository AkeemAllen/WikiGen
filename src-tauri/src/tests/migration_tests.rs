use crate::migrations::{gather_migrations, run_migrations};

#[test]
fn test_gather_migrations() {
    let resources_path = std::path::PathBuf::from("/Applications/WikiGen.app/Contents/Resources");
    let base_path =
        std::path::PathBuf::from("/Users/akeemallen/Library/Application Support/com.wikigen.dev");

    match gather_migrations(&base_path, &resources_path) {
        Ok(migrations) => migrations,
        Err(err) => panic!("Error checking for migrations: {}", err),
    };
}

#[tokio::test]
async fn test_run_migrations() {
    let resources_path = std::path::PathBuf::from("/Applications/WikiGen.app/Contents/Resources");

    let base_path =
        std::path::PathBuf::from("/Users/akeemallen/Library/Application Support/com.wikigen.dev");

    let migrations = match gather_migrations(&base_path, &resources_path) {
        Ok(migrations) => migrations,
        Err(err) => panic!("Error checking for migrations: {}", err),
    };

    match run_migrations(migrations, &base_path, &resources_path).await {
        Ok(migrations) => migrations,
        Err(err) => panic!("Error checking for migrations: {}", err),
    };
}
