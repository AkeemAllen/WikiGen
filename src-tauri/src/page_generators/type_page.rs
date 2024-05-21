use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{
    helpers::{capitalize, get_pokemon_dex_formatted_name},
    structs::{
        mkdocs_structs::{MKDocsConfig, Navigation},
        pokemon_structs::Pokemon,
    },
};

type ModifiedPokemon = HashMap<String, ModifiedPokemonDetails>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ModifiedPokemonDetails {
    id: usize,
    types: Types,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Types {
    original: Vec<String>,
    modified: Vec<String>,
}

pub fn generate_type_page(wiki_name: &str, base_path: PathBuf) -> Result<String, String> {
    let modified_pokemon_json_file_path = base_path
        .join(wiki_name)
        .join("data")
        .join("modifications")
        .join("modified_pokemon.json");
    let modified_pokemon_file = File::open(&modified_pokemon_json_file_path).unwrap();
    let modified_pokemon: ModifiedPokemon = serde_json::from_reader(modified_pokemon_file).unwrap();

    let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
    let mkdocs_yaml_file = File::open(&mkdocs_yaml_file_path).unwrap();
    let mut mkdocs_config: MKDocsConfig = serde_yaml::from_reader(mkdocs_yaml_file).unwrap();

    let mut type_changes_file = File::create(
        base_path
            .join(wiki_name)
            .join("dist")
            .join("docs")
            .join("type_changes.md"),
    )
    .unwrap();

    let mut type_changes_markdown = String::new();
    for (pokemon_name, modification_details) in modified_pokemon.iter() {
        if modification_details.types.modified.len() > 0 {
            let type_change = format!(
                "| {} | {} | {} |\n",
                get_markdown_entry_for_pokemon(wiki_name, pokemon_name, modification_details.id),
                get_type_images(modification_details.types.original.clone()),
                get_type_images(modification_details.types.modified.clone())
            );
            type_changes_markdown.push_str(&type_change);
        }
    }

    type_changes_file
        .write_all(
            format!(
                "| Pokemon | Old Type | New Type |
            | :--: | :-- | :-- |
            {}
            ",
                type_changes_markdown
            )
            .as_bytes(),
        )
        .unwrap();

    let mut specific_changes: HashMap<String, Navigation> = HashMap::new();

    /*
    Collecting the hashmap with the Specific Changes key here so it's easier to
    check for existing values further down.

    Note: this is probably not the most elegant way of collecting this value. However,
    because the navigation object in the mkdocs_config is so complex, I may not have a choice
    here.
     */
    if let Some(pokemon_nav) = mkdocs_config.nav[1].get("Pokemon") {
        if let Navigation::Array(pokemon_nav) = pokemon_nav {
            for nav_item_map in pokemon_nav.iter() {
                if let Navigation::Map(nav_item) = nav_item_map {
                    if nav_item.contains_key("Specific Changes") {
                        specific_changes = nav_item.clone();
                    }
                }
            }
        }
    }

    let mut type_changes_entry: HashMap<String, Navigation> = HashMap::new();
    type_changes_entry.insert(
        "Type Changes".to_string(),
        Navigation::String("type_changes.md".to_string()),
    );

    if let Some(pokemon_nav) = mkdocs_config.nav[1].get_mut("Pokemon") {
        *pokemon_nav = Navigation::Array(vec![
            Navigation::Map(type_changes_entry),
            Navigation::Map(specific_changes),
        ])
    }

    fs::write(
        mkdocs_yaml_file_path,
        serde_yaml::to_string(&mkdocs_config).unwrap(),
    )
    .unwrap();

    Ok("Type page generated".to_string())
}

pub fn get_markdown_entry_for_pokemon(
    wiki_name: &str,
    pokemon_name: &str,
    pokemon_id: usize,
) -> String {
    let dex_number_file_name = get_pokemon_dex_formatted_name(pokemon_id);
    return format!(
        "![{}](img/pokemon/{}.png)<br/> [{}](/{}/pokemon/{})",
        pokemon_name,
        dex_number_file_name,
        capitalize(&pokemon_name),
        wiki_name,
        dex_number_file_name,
    );
}

fn get_type_images(types: Vec<String>) -> String {
    let first_type = types.get(0).unwrap();
    if types.len() == 1 {
        return format!("![{}](img/types/{}.png)", first_type, first_type);
    }
    let second_type = types.get(1).unwrap();
    return format!(
        "![{}](img/types/{}.png)<br/>![{}](img/types/{}.png)",
        first_type, first_type, second_type, second_type
    );
}
