// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod helpers;
mod page_generators;
mod structs;
mod wiki_preparation;

use helpers::mkdocs_process::{check_process_status, kill_mkdocs_process, spawn_mkdocs_process};
use page_generators::pokemon_pages::generate_pokemon_pages_in_range;
use wiki_preparation::create_wiki::create_wiki;
use wiki_preparation::game_routes::create_new_route;
use wiki_preparation::prepare_data::{download_and_prep_pokemon_data, download_pokemon_sprites};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_wiki,
            download_and_prep_pokemon_data,
            spawn_mkdocs_process,
            kill_mkdocs_process,
            check_process_status,
            generate_pokemon_pages_in_range,
            download_pokemon_sprites,
            create_new_route,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
