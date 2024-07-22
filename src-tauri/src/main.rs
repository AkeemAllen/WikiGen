// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod helpers;
mod page_generators;
mod structs;
mod tests;
mod wiki_preparation;

use std::fs::File;

use helpers::json_conversion::{
    convert_abilities_to_sqlite, convert_items_to_sqlite, convert_moves_to_sqlite,
    convert_natures_to_sqlite, convert_pokemon_movesets_to_sqlite, convert_pokemon_to_sqlite,
};
use helpers::mkdocs_process::{check_process_status, kill_mkdocs_process, spawn_mkdocs_process};
use page_generators::ability_page::generate_ability_page;
use page_generators::game_routes::{
    generate_route_pages_with_handle, generate_single_route_page_with_handle,
};
use page_generators::item_page::generate_item_page;
use page_generators::nature_page::generate_nature_page;
use page_generators::pokemon_pages::generate_pokemon_pages_from_list;
use tauri_plugin_sql;
use wiki_preparation::backup_wiki::backup_wiki;
use wiki_preparation::create_wiki::create_wiki;
use wiki_preparation::prepare_data::{download_and_prep_pokemon_data, download_pokemon_sprites};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let base_path = app.path_resolver().app_data_dir().unwrap();
            match base_path.join("initial.db").try_exists() {
                Ok(true) => {
                    println!("Database already exists");
                }
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
            }
            Ok(())
        })
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            create_wiki,
            download_and_prep_pokemon_data,
            spawn_mkdocs_process,
            kill_mkdocs_process,
            check_process_status,
            generate_pokemon_pages_from_list,
            download_pokemon_sprites,
            generate_route_pages_with_handle,
            generate_single_route_page_with_handle,
            backup_wiki,
            generate_item_page,
            generate_nature_page,
            generate_ability_page,
            convert_items_to_sqlite,
            convert_abilities_to_sqlite,
            convert_natures_to_sqlite,
            convert_moves_to_sqlite,
            convert_pokemon_to_sqlite,
            convert_pokemon_movesets_to_sqlite
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
