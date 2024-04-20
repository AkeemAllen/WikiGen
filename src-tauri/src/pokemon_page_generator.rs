use std::{fs::File, path::Path};

#[tauri::command]
pub async fn generate_pokemon_pages_in_range(range_start: usize, range_end: usize) {
    for dex_number in range_start..=range_end {
        println!("Generating Pokemon Page for Dex Number: {}", dex_number);
        let base_path = Path::new(dir).join(wiki_name);
        let pokemon_path = base_path.join("data").join("pokemon.json");

        let pokemon_file = File::open(&pokemon_path).unwrap();

        let mut pokemon: Pokemon = serde_json::from_reader(pokemon_file).unwrap();
        // let pokemon = get_pokemon_by_dex_number(dex_number);
        // let pokemon_page = generate_pokemon_page(pokemon);
        // let file_path = format!("dist/docs/pokemon/{}.md", pokemon.name);
        // fs::write(file_path, pokemon_page).unwrap();
    }
}
