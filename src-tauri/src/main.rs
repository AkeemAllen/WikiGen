// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod database;
mod helpers;
mod logger;
mod migrations;
mod page_generators;
mod structs;
mod tests;
mod wiki_preparation;

use helpers::mkdocs_process::{check_process_status, kill_mkdocs_process, spawn_mkdocs_process};
use page_generators::ability_page::generate_ability_page_with_handle;
use page_generators::game_routes::{
    delete_route_page_from_mkdocs, generate_route_pages_with_handle,
};
use page_generators::item_page::generate_items_page_with_handle;
use page_generators::nature_page::generate_nature_page_with_handle;
use page_generators::pokemon_pages::{
    generate_pokemon_pages_from_list, remove_pokemon_page_with_old_dex_number,
    update_pokemon_pages_with_stripped_name_with_handle,
};
use tauri_plugin_sql;
use wiki_preparation::backup_wiki::backup_wiki;
use wiki_preparation::create_wiki::create_wiki;
use wiki_preparation::deploy_wiki::deploy_wiki;

use migrations::run_migrations;
use wiki_preparation::yaml_declaration::update_yaml;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            create_wiki,
            spawn_mkdocs_process,
            kill_mkdocs_process,
            check_process_status,
            generate_pokemon_pages_from_list,
            generate_route_pages_with_handle,
            backup_wiki,
            deploy_wiki,
            generate_items_page_with_handle,
            generate_nature_page_with_handle,
            generate_ability_page_with_handle,
            update_yaml,
            delete_route_page_from_mkdocs,
            remove_pokemon_page_with_old_dex_number,
            update_pokemon_pages_with_stripped_name_with_handle,
            run_migrations
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
