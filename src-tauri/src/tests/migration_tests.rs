use crate::{database::get_sqlite_connection, migrations::create_categories_table};

#[tokio::test]
async fn test_create_categories_table() {
    let base_path =
        std::path::PathBuf::from("/Users/akeemallen/Library/Application Support/com.wikigen.dev");

    let sqlite_file_path = base_path.join("testing").join("testing.db");

    let conn = get_sqlite_connection(sqlite_file_path).await.unwrap();
    create_categories_table(&conn).await.unwrap();
}
