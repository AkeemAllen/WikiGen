use std::{fs::File, path::Path};

use tauri::AppHandle;

use crate::structs::pokemon_structs::Pokemon;

#[tauri::command]
pub async fn generate_pokemon_pages_in_range(
    range_start: usize,
    range_end: usize,
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    for dex_number in range_start..=range_end {
        println!("Generating Pokemon Page for Dex Number: {}", dex_number);
        let base_path = app_handle.path_resolver().app_data_dir().unwrap();
        let pokemon_path = base_path.join(wiki_name).join("data").join("pokemon.json");

        let pokemon_file = File::open(&pokemon_path).unwrap();

        let mut pokemon: Pokemon = serde_json::from_reader(pokemon_file).unwrap();
        // let pokemon = get_pokemon_by_dex_number(dex_number);
        // let pokemon_page = generate_pokemon_page(pokemon);
        // let file_path = format!("dist/docs/pokemon/{}.md", pokemon.name);
        // fs::write(file_path, pokemon_page).unwrap();
    }
    return Ok("Pokemon Pages Generated".to_string());
}
