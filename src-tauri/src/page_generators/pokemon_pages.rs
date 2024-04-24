use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

use tauri::AppHandle;

use crate::{
    helpers::capitalize,
    structs::{
        mkdocs_structs::{MKDocsConfig, Navigation},
        pokemon_structs::Pokemon,
    },
};

#[tauri::command]
pub async fn generate_pokemon_pages_in_range(
    range_start: usize,
    range_end: usize,
    wiki_name: &str,
    app_handle: AppHandle,
) -> Result<String, String> {
    let base_path = app_handle.path_resolver().app_data_dir().unwrap();
    let docs_path = base_path.join(wiki_name).join("dist").join("docs");

    let pokemon_json_file_path = base_path.join(wiki_name).join("data").join("pokemon.json");
    let pokemon_file = File::open(&pokemon_json_file_path).unwrap();
    let pokemon: Pokemon = serde_json::from_reader(pokemon_file).unwrap();

    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mkdocs_yaml_file = File::open(&mkdocs_yaml_file_path).unwrap();
    let mut mkdocs_config: MKDocsConfig = serde_yaml::from_reader(mkdocs_yaml_file).unwrap();

    let mut specific_changes = HashMap::new();

    /*
    Collecting the hashmap with the Specific Changes key here so it's easier to
    check for existing values further down.

    Note: this is probably not the most elegant way of collecting this value. However,
    because the navigation object in the mkdocs_config is so complex, I may not have a choice
    here.
     */
    if let Some(pokemon_nav) = mkdocs_config.nav[1].get("Pokemon") {
        if let Navigation::Array(pokemon_nav) = pokemon_nav {
            if let Navigation::Map(pokemon_nav) = &pokemon_nav[0] {
                specific_changes = pokemon_nav.clone();
            }
        }
    }

    for dex_number in range_start..=range_end {
        let pokemon_data = pokemon.pokemon.get(&(dex_number as u32));

        let data = match pokemon_data {
            Some(pokemon_data) => pokemon_data,
            None => {
                println!("Pokemon Data not found for dex number: {:?}", dex_number);
                continue;
            }
        };

        let mut pokedex_markdown_file_name = format!("00{}", dex_number);
        if dex_number >= 10 {
            pokedex_markdown_file_name = format!("0{}", dex_number);
        }
        if dex_number >= 100 {
            pokedex_markdown_file_name = format!("{}", dex_number);
        }

        let pokemon_markdown_file = File::create(
            docs_path
                .join("pokemon")
                .join(format!("{}.md", pokedex_markdown_file_name)),
        );

        let mut markdown_file = match pokemon_markdown_file {
            Ok(file) => file,
            Err(e) => {
                println!("Error creating file: {:?}", e);
                continue;
            }
        };
        markdown_file
            .write_all(
                format!(
                    "![{}](../img/pokemon/{}.png)\n\n",
                    data.name, pokedex_markdown_file_name
                )
                .as_bytes(),
            )
            .unwrap();

        markdown_file.write_all(b"## Types\n\n").unwrap();

        markdown_file
            .write_all(create_type_table(&data.types).as_bytes())
            .unwrap();

        let mut specific_change_entry = HashMap::new();
        let entry_key = format!(
            "{} - {}",
            pokedex_markdown_file_name,
            capitalize::capitalize(&data.name)
        );
        specific_change_entry.insert(
            entry_key.clone(),
            Navigation::String(format!("pokemon/{}.md", pokedex_markdown_file_name)),
        );

        /*
        This block of code check if the specific_change_entry exists in
        specific changes already. If not, then we push it to the array.

        Note: Similar to gathering the specific changes above, the complexity
        of the nesting requires all of the steps here.
         */
        if let Some(change) = specific_changes.get_mut("Specific Changes") {
            if let Navigation::Array(entries) = change {
                let mut entry_exists = false;
                for entry in &mut *entries {
                    if let Navigation::Map(entry_map) = entry {
                        if entry_map.contains_key(&entry_key.clone()) {
                            entry_exists = true;
                            break;
                        }
                    }
                }
                if !entry_exists {
                    entries.push(Navigation::Map(specific_change_entry))
                }
            }
        }
    }

    if let Some(pokemon_nav) = mkdocs_config.nav[1].get_mut("Pokemon") {
        *pokemon_nav = Navigation::Array(vec![Navigation::Map(specific_changes)]);
    };

    fs::write(
        mkdocs_yaml_file_path,
        serde_yaml::to_string(&mkdocs_config).unwrap(),
    )
    .unwrap();

    return Ok("Pokemon Pages Generated".to_string());
}

fn create_type_table(types: &Vec<String>) -> String {
    let type_images: Vec<String> = types
        .iter()
        .map(|_type| format!("![{}](../img/types/{}.png)", _type, _type))
        .collect();
    return format!(
        "| Version | Type |
            | :--: | ----: |
            | Classic | {} |
        ",
        type_images.join(" ")
    );
}
