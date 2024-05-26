// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod helpers;
mod page_generators;
mod structs;
mod wiki_preparation;

use helpers::mkdocs_process::{check_process_status, kill_mkdocs_process, spawn_mkdocs_process};
use page_generators::game_routes::{
    generate_route_pages_with_handle, generate_single_route_page_with_handle,
};
use page_generators::pokemon_pages::{
    generate_pokemon_pages_from_list, generate_pokemon_pages_from_range,
};
use wiki_preparation::create_wiki::create_wiki;
use wiki_preparation::prepare_data::{download_and_prep_pokemon_data, download_pokemon_sprites};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_wiki,
            download_and_prep_pokemon_data,
            spawn_mkdocs_process,
            kill_mkdocs_process,
            check_process_status,
            generate_pokemon_pages_from_range,
            generate_pokemon_pages_from_list,
            download_pokemon_sprites,
            generate_route_pages_with_handle,
            generate_single_route_page_with_handle
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
