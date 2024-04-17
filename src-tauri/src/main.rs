// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod create_wiki;
mod prepare_data;
mod utils;
mod yaml_declaration;

use create_wiki::create_wiki;
use prepare_data::download_and_prep_pokemon_data;
use utils::{check_process_status, kill_mkdocs_process, spawn_mkdocs_process};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_wiki,
            download_and_prep_pokemon_data,
            spawn_mkdocs_process,
            kill_mkdocs_process,
            check_process_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
