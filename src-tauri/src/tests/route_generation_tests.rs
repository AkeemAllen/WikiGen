use serde_yaml::Value;

use crate::{database::get_mkdocs_config, page_generators::game_routes::generate_route_pages};

#[test]
// Route Page is created and present in the mkdocs.yml file
fn test_generate_route_page_created() {
    let base_path =
        std::path::PathBuf::from("/Users/akeemallen/Library/Application Support/com.wikigen.dev");
    let resource_path = std::path::PathBuf::from("/Applications/WikiGen.app/Contents/Resources");

    let result = generate_route_pages("testing", &base_path, &resource_path, &["Route 1"]);

    assert!(result.unwrap() == "Route Page Generated".to_string());
    let generated_path = base_path
        .join("testing")
        .join("dist")
        .join("docs")
        .join("routes")
        .join("Route 1.md");
    assert!(generated_path.exists());

    // let generated_file = match read_to_string(&generated_path) {
    //     Ok(file) => file,
    //     Err(err) => panic!("Failed to read generated file: {}", err),
    // };

    // let snapshot = match read_to_string(
    //     base_path
    //         .join("testing")
    //         .join("snapshots")
    //         .join("generated_route-Route_1.md"),
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

    let mut mkdocs_routes: &mut Vec<Value> = &mut Vec::new();

    let nav_entries = mkdocs_config.nav.as_sequence_mut().unwrap();
    for entry in nav_entries {
        let map_entries = entry.as_mapping_mut().unwrap();
        match map_entries.get_mut(Value::String("Routes".to_string())) {
            Some(map_entry) => {
                mkdocs_routes = map_entry.as_sequence_mut().unwrap();
            }
            None => {}
        }
    }

    let mut page_entry_exists = false;
    let mut page_position = 0;
    for (index, page_entry) in mkdocs_routes.iter_mut().enumerate() {
        if page_entry.as_mapping().unwrap().contains_key("Route 1") {
            page_entry_exists = true;
            page_position = index;
            break;
        }
    }

    assert!(page_entry_exists);

    // Clean up
    mkdocs_routes.remove(page_position);
    std::fs::remove_file(generated_path).unwrap();

    match std::fs::write(
        &mkdocs_yaml_file_path,
        serde_yaml::to_string(&mut mkdocs_config).unwrap(),
    ) {
        Ok(_) => {}
        Err(err) => panic!("Failed to update mkdocs yaml: {}", err),
    };
}
