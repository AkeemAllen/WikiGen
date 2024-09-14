use serde_yaml::Value;

use crate::{
    database::get_mkdocs_config,
    page_generators::pokemon_pages::generate_pokemon_pages,
    structs::pokemon_structs::{DBPokemon, PokemonMove},
};

#[test]
fn test_generate_pokemon_page() {
    // Check that page is created and entry is in mkdocs.
    // Remove once complete
    let base_path =
        std::path::PathBuf::from("/Users/akeemallen/Library/Application Support/com.wikigen.dev");
    let resource_path = std::path::PathBuf::from("/Applications/WikiGen.app/Contents/Resources");
    let pokemon_list: Vec<DBPokemon> = vec![DBPokemon {
        id: 1,
        name: "bulbasaur".to_string(),
        dex_number: 1,
        types: "grass, poison".to_string(),
        ability_1: Some("overgrow".to_string()),
        ability_2: Some("chlorophyll".to_string()),
        hidden_ability: None,
        a1_effect: Some("An Effect".to_string()),
        a2_effect: Some("An Effect".to_string()),
        h3_effect: None,
        hp: 45,
        attack: 49,
        defense: 49,
        sp_attack: 65,
        sp_defense: 65,
        speed: 45,
        evolution_method: "no_change".to_string(),
        evolution_item: None,
        evolution_level: None,
        evolution_other: None,
        evolved_pokemon: None,
        render: "true".to_string(),
    }];
    let moveset: Vec<PokemonMove> = vec![PokemonMove {
        pokemon: 1,
        move_id: 1,
        learn_method: "level_up".to_string(),
        level_learned: Some(1),
        move_name: "tackle".to_string(),
        move_type: Some("normal".to_string()),
        power: Some(40),
        accuracy: Some(100),
        pp: Some(35),
        damage_class: "physical".to_string(),
        machine_name: None,
    }];
    let result = generate_pokemon_pages(
        "testing",
        &pokemon_list,
        &moveset,
        &base_path,
        &resource_path,
    );
    assert!(result.unwrap() == "Pokemon Pages Generated".to_string());
    let generated_path = base_path
        .join("testing")
        .join("dist")
        .join("docs")
        .join("pokemon")
        .join("001-bulbasaur.md");
    assert!(generated_path.exists());

    let mkdocs_yaml_file_path = base_path.join("testing").join("dist").join("mkdocs.yml");
    let mut mkdocs_config = match get_mkdocs_config(&mkdocs_yaml_file_path) {
        Ok(config) => config,
        Err(err) => {
            panic!("Failed to get mkdocs config: {}", err);
        }
    };

    let mut mkdocs_pokemon: &mut Vec<Value> = &mut Vec::new();

    let nav_entries = mkdocs_config.nav.as_sequence_mut().unwrap();
    for entry in nav_entries {
        let map_entries = entry.as_mapping_mut().unwrap();
        match map_entries.get_mut(Value::String("Pokemon".to_string())) {
            Some(map_entry) => {
                mkdocs_pokemon = map_entry.as_sequence_mut().unwrap();
            }
            None => {}
        }
    }

    let mut page_entry_exists = false;
    let mut page_position = 0;
    for (index, page_entry) in mkdocs_pokemon.iter_mut().enumerate() {
        if page_entry
            .as_mapping()
            .unwrap()
            .contains_key("001 - Bulbasaur")
        {
            page_entry_exists = true;
            page_position = index;
            break;
        }
    }

    assert!(page_entry_exists);

    // Clean up
    mkdocs_pokemon.remove(page_position);
    std::fs::remove_file(generated_path).unwrap();

    match std::fs::write(
        &mkdocs_yaml_file_path,
        serde_yaml::to_string(&mut mkdocs_config).unwrap(),
    ) {
        Ok(_) => {}
        Err(err) => panic!("Failed to update mkdocs yaml: {}", err),
    };
}
