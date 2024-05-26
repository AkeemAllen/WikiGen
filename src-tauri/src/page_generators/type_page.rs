use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use serde_yaml::{Mapping, Value};

use crate::{
    helpers::{capitalize, get_pokemon_dex_formatted_name},
    structs::mkdocs_structs::MKDocsConfig,
};

pub fn generate_type_page(wiki_name: &str, base_path: PathBuf) -> Result<String, String> {
    let modified_pokemon_json_file_path = base_path
        .join(wiki_name)
        .join("data")
        .join("modifications")
        .join("modified_pokemon.json");
    let modified_pokemon_file = File::open(&modified_pokemon_json_file_path).unwrap();
    let modified_pokemon: super::ModifiedPokemon =
        serde_json::from_reader(modified_pokemon_file).unwrap();

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

    let nav_entries = mkdocs_config.nav.as_sequence_mut().unwrap();
    let mut type_page_exists = false;
    let mut page_index = 0;
    for (index, entry) in nav_entries.iter_mut().enumerate() {
        let map_entries = entry.as_mapping_mut().unwrap();
        if let Some(_) = map_entries.get_mut(Value::String("Type Changes".to_string())) {
            type_page_exists = true;
            page_index = index;
            break;
        }
    }

    if modified_pokemon.is_empty() {
        if !type_page_exists {
            return Ok("No Types to generate".to_string());
        }

        fs::remove_file(
            base_path
                .join(wiki_name)
                .join("dist")
                .join("docs")
                .join("type_changes.md"),
        )
        .unwrap();
        mkdocs_config
            .nav
            .as_sequence_mut()
            .unwrap()
            .remove(page_index);
        fs::write(
            &mkdocs_yaml_file_path,
            serde_yaml::to_string(&mkdocs_config).unwrap(),
        )
        .unwrap();
    }

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

    let mut type_changes = Mapping::new();
    type_changes.insert(
        Value::String("Type Changes".to_string()),
        Value::String("type_changes.md".to_string()),
    );

    if type_page_exists {
        return Ok("Types Page Updated".to_string());
    }

    mkdocs_config
        .nav
        .as_sequence_mut()
        .unwrap()
        .insert(1, Value::Mapping(type_changes));

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
