// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod database;
mod helpers;
mod migrations;
mod page_generators;
mod structs;
mod tests;
mod wiki_preparation;

use std::fs::{self, File};

use helpers::mkdocs_process::{check_process_status, kill_mkdocs_process, spawn_mkdocs_process};
use page_generators::ability_page::generate_ability_page_with_handle;
use page_generators::game_routes::{
    delete_route_page_from_mkdocs, generate_route_pages_with_handle,
};
use page_generators::item_page::{
    generate_item_changes_page_with_handle, generate_item_location_page,
};
use page_generators::nature_page::generate_nature_page_with_handle;
use page_generators::pokemon_pages::{
    generate_pokemon_pages_from_list, remove_pokemon_page_with_old_dex_number,
};
use tauri_plugin_sql;
use wiki_preparation::backup_wiki::backup_wiki;
use wiki_preparation::create_wiki::create_wiki;

use migrations::run_migrations;
use wiki_preparation::yaml_declaration::update_yaml;

fn main() {
    let app = tauri::Builder::default()
        .setup(|app| {
            let base_path = app.path_resolver().app_data_dir().unwrap();
            match base_path.join("initial.db").try_exists() {
                Ok(false) => {
                    println!("Creating initial database");
                    match File::create(base_path.join("initial.db")) {
                        Ok(_) => {
                            println!("Database created");
                        }
                        Err(_) => {
                            println!("Error creating initial.db");
                        }
                    };
                }
                Err(_) => {
                    println!("Error Checking for initial.db");
                }
                _ => {}
            }
            Ok(())
        })
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            create_wiki,
            spawn_mkdocs_process,
            kill_mkdocs_process,
            check_process_status,
            generate_pokemon_pages_from_list,
            generate_route_pages_with_handle,
            backup_wiki,
            generate_item_changes_page_with_handle,
            generate_nature_page_with_handle,
            generate_ability_page_with_handle,
            update_yaml,
            delete_route_page_from_mkdocs,
            generate_item_location_page,
            remove_pokemon_page_with_old_dex_number
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::Ready => {
            let base_path = _app_handle.path_resolver().app_data_dir().unwrap();
            let resource_path = _app_handle.path_resolver().resource_dir().unwrap();
            match tauri::async_runtime::block_on(run_migrations(&base_path, &resource_path)) {
                Ok(_) => {}
                Err(err) => {
                    let migration_error_file = _app_handle
                        .path_resolver()
                        .app_data_dir()
                        .unwrap()
                        .join("migration_error.txt");
                    fs::write(
                        migration_error_file,
                        format!("Error running database migrations: {}", err),
                    )
                    .expect("Unable to write file");
                }
            }
        }
        _ => {}
    });
}
