use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use crate::structs::{
    mkdocs_structs::{MKDocsConfig, Navigation},
    pokemon_structs::EvolutionMethod,
};

use super::type_page::get_markdown_entry_for_pokemon;

pub fn generate_evolution_page(wiki_name: &str, base_path: PathBuf) -> Result<String, String> {
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

    let mut evolution_changes_file = File::create(
        base_path
            .join(wiki_name)
            .join("dist")
            .join("docs")
            .join("evolution_changes.md"),
    )
    .unwrap();

    let mut evolution_changes_markdown = String::new();
    let mut evolution_level = String::new();
    let mut evolution_other = String::new();
    let mut evolution_item_iteraction = String::new();

    for (pokemon_name, modification_details) in modified_pokemon.iter() {
        let evolution_details = modification_details.evolution.clone();
        match evolution_details.method {
            EvolutionMethod::LevelUp => {
                let entry = format!(
                    "| {} | {} | {} |\n",
                    get_markdown_entry_for_pokemon(
                        wiki_name,
                        pokemon_name,
                        modification_details.id
                    ),
                    evolution_details.level,
                    get_markdown_entry_for_pokemon(
                        wiki_name,
                        &evolution_details.evolves_to.pokemon_name,
                        evolution_details.evolves_to.id
                    )
                );
                if evolution_level.is_empty() {
                    evolution_level.push_str(&format!(
                        "| Base Pokemon | Level | Evolution |
                                | :--: | :-- | :--: |
                                "
                    ))
                }
                evolution_level.push_str(&entry);
            }
            EvolutionMethod::Other => {
                let entry = format!(
                    "| {} | {} | {} |\n",
                    get_markdown_entry_for_pokemon(
                        wiki_name,
                        pokemon_name,
                        modification_details.id
                    ),
                    &evolution_details.other,
                    get_markdown_entry_for_pokemon(
                        wiki_name,
                        pokemon_name,
                        modification_details.id
                    )
                );
                if evolution_other.is_empty() {
                    evolution_other.push_str(&format!(
                        "| Base Pokemon | Method | Evolution |
                                    | :--: | :-- | :--: |
                                    "
                    ))
                }
                evolution_other.push_str(&entry)
            }
            EvolutionMethod::Item => {
                let entry = format!(
                    "| {} | {} | {} |\n",
                    get_markdown_entry_for_pokemon(
                        wiki_name,
                        pokemon_name,
                        modification_details.id
                    ),
                    evolution_details.item,
                    get_markdown_entry_for_pokemon(
                        wiki_name,
                        pokemon_name,
                        modification_details.id
                    )
                );
                if evolution_item_iteraction.is_empty() {
                    evolution_item_iteraction.push_str(&format!(
                        "| Base Pokemon | Item | Evolution |
                                    | :--: | :-- | :--: |
                                    "
                    ))
                }
                evolution_item_iteraction.push_str(&entry)
            }
            EvolutionMethod::NoChange => {}
        }
    }

    if !evolution_level.is_empty() {
        let entry = format!("{}\n", evolution_level);
        evolution_changes_markdown.push_str(&entry);
    }
    if !evolution_item_iteraction.is_empty() {
        let entry = format!("{}\n", evolution_item_iteraction);
        evolution_changes_markdown.push_str(&entry);
    }
    if !evolution_other.is_empty() {
        let entry = format!("{}\n", evolution_other);
        evolution_changes_markdown.push_str(&entry);
    }

    evolution_changes_file
        .write_all(format!("{}", evolution_changes_markdown).as_bytes())
        .unwrap();

    let mut specific_changes: HashMap<String, Navigation> = HashMap::new();
    let mut type_changes: HashMap<String, Navigation> = HashMap::new();

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
                    if nav_item.contains_key("Type Changes") {
                        type_changes = nav_item.clone();
                    }
                }
            }
        }
    }

    let mut evolution_changes_entry = HashMap::new();

    evolution_changes_entry.insert(
        "Evolution Changes".to_string(),
        Navigation::String("evolution_changes.md".to_string()),
    );

    if let Some(pokemon_nav) = mkdocs_config.nav[1].get_mut("Pokemon") {
        *pokemon_nav = Navigation::Array(vec![
            Navigation::Map(type_changes),
            Navigation::Map(evolution_changes_entry),
            Navigation::Map(specific_changes),
        ])
    }

    fs::write(
        mkdocs_yaml_file_path,
        serde_yaml::to_string(&mkdocs_config).unwrap(),
    )
    .unwrap();

    Ok("Evolution Pages Generated".to_string())
}