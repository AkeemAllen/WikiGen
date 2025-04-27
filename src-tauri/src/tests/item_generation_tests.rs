use std::fs::read_to_string;

use serde_yaml::Value;

use crate::{
    database::get_mkdocs_config,
    page_generators::item_page::{generate_items_page, Item, ItemLocation},
};

#[test]
// Ability Page is created and present in the mkdocs.yml file
fn test_generate_items_page_created() {
    let base_path =
        std::path::PathBuf::from("/Users/akeemallen/Library/Application Support/com.wikigen.dev");

    let items = vec![
        Item {
            name: "leftovers".to_string(),
            effect: "leftovers".to_string(),
            category: "Held".to_string(),
            is_modified: 1,
            is_new: 0,
        },
        Item {
            name: "leftovers".to_string(),
            effect: "leftovers".to_string(),
            category: "Held".to_string(),
            is_modified: 1,
            is_new: 0,
        },
        Item {
            name: "poke-ball".to_string(),
            effect: "pokeball".to_string(),
            category: "Pokeball".to_string(),
            is_modified: 0,
            is_new: 1,
        },
        Item {
            name: "poke-ball".to_string(),
            effect: "pokeball".to_string(),
            category: "Pokeball".to_string(),
            is_modified: 0,
            is_new: 1,
        },
    ];

    let item_locations = vec![ItemLocation {
        item_name: "leftovers".to_string(),
        route: "Route 2".to_string(),
        specific_location: Some("".to_string()),
        method: Some("".to_string()),
        requirements: Some("Test".to_string()),
    }];

    let result = generate_items_page("testing", &base_path, &items, &item_locations);
    println!("{:?}", result);
    // assert!(result.unwrap() == "Items Page Generated".to_string());
    let generated_path = base_path
        .join("testing")
        .join("dist")
        .join("docs")
        .join("item_information.md");
    assert!(generated_path.exists());

    // let generated_file = match read_to_string(&generated_path) {
    //     Ok(file) => file,
    //     Err(err) => panic!("Failed to read generated file: {}", err),
    // };

    // let snapshot = match read_to_string(
    //     base_path
    //         .join("testing")
    //         .join("snapshots")
    //         .join("generated_item_changes.md"),
    // ) {
    //     Ok(snapshot) => snapshot,
    //     Err(err) => panic!("Failed to read snapshot file: {}", err),
    // };

    // assert_eq!(generated_file, snapshot);

    let mkdocs_yaml_file_path = base_path.join("testing").join("dist").join("mkdocs.yml");
    let mut mkdocs_config = match get_mkdocs_config(&mkdocs_yaml_file_path) {
        Ok(config) => config,
        Err(err) => {
            panic!("Failed to get mkdocs config: {}", err);
        }
    };

    let nav_entries = mkdocs_config.nav.as_sequence_mut().unwrap();

    let mut page_entry_exists = false;
    let mut page_index = 0;
    for (index, entry) in nav_entries.iter_mut().enumerate() {
        let map_entries = entry.as_mapping_mut().unwrap();
        if let Some(_) = map_entries.get_mut(Value::String("Item Information".to_string())) {
            page_entry_exists = true;
            page_index = index;
            break;
        }
    }

    assert!(page_entry_exists);

    // Clean up
    // mkdocs_config
    //     .nav
    //     .as_sequence_mut()
    //     .unwrap()
    //     .remove(page_index);
    // std::fs::remove_file(generated_path).unwrap();

    // match std::fs::write(
    //     &mkdocs_yaml_file_path,
    //     serde_yaml::to_string(&mut mkdocs_config).unwrap(),
    // ) {
    //     Ok(_) => {}
    //     Err(err) => panic!("Failed to update mkdocs yaml: {}", err),
    // };
}
