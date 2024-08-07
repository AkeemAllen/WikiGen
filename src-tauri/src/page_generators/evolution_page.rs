// use std::{
//     fs::{self, File},
//     io::Write,
//     path::PathBuf,
// };

// use serde_yaml::{Mapping, Value};

// use crate::structs::{
//     mkdocs_structs::MKDocsConfig,
//     pokemon_structs::{Evolution, EvolutionMethod},
// };

// use super::type_page::get_markdown_entry_for_pokemon;

// pub fn generate_evolution_page(wiki_name: &str, base_path: PathBuf) -> Result<String, String> {
// let modified_pokemon_json_file_path = base_path
//     .join(wiki_name)
//     .join("data")
//     .join("modifications")
//     .join("modified_pokemon.json");
// let modified_pokemon_file = File::open(&modified_pokemon_json_file_path).unwrap();
// let modified_pokemon: super::ModifiedPokemon =
//     match serde_json::from_reader(modified_pokemon_file) {
//         Ok(file) => file,
//         Err(err) => {
//             return Err(format!(
//                 "Failed to read modified pokemon json file: {}",
//                 err
//             ))
//         }
//     };

// let mkdocs_yaml_file_path = base_path.join(wiki_name).join("dist").join("mkdocs.yml");
// let mkdocs_yaml_file = File::open(&mkdocs_yaml_file_path).unwrap();
// let mut mkdocs_config: MKDocsConfig = match serde_yaml::from_reader(mkdocs_yaml_file) {
//     Ok(file) => file,
//     Err(err) => return Err(format!("Failed to read mkdocs yaml file: {}", err)),
// };

// let mut evolution_changes_file = match File::create(
//     base_path
//         .join(wiki_name)
//         .join("dist")
//         .join("docs")
//         .join("evolution_changes.md"),
// ) {
//     Ok(file) => file,
//     Err(err) => return Err(format!("Failed to create evolution changes file: {}", err)),
// };

// let nav_entries = mkdocs_config.nav.as_sequence_mut().unwrap();
// let mut evolution_page_exists = false;
// let mut page_index = 0;
// for (index, entry) in nav_entries.iter_mut().enumerate() {
//     let map_entries = entry.as_mapping_mut().unwrap();
//     if let Some(_) = map_entries.get_mut(Value::String("Evolution Changes".to_string())) {
//         evolution_page_exists = true;
//         page_index = index;
//         break;
//     }
// }

// let mut evolution_changes_markdown = String::new();
// let mut evolution_level = String::new();
// let mut evolution_other = String::new();
// let mut evolution_item_iteraction = String::new();

// for (pokemon_name, modification_details) in modified_pokemon.iter() {
//     let evolution_details = modification_details.evolution.clone();
//     if let EvolutionMethod::NoChange = evolution_details.method {
//         continue;
//     }

//     let entry = format!(
//         "| {} | {} | {} |\n",
//         get_markdown_entry_for_pokemon(wiki_name, pokemon_name, modification_details.id),
//         get_level_item_method(&evolution_details),
//         get_markdown_entry_for_pokemon(
//             wiki_name,
//             &evolution_details.evolves_to.pokemon_name,
//             evolution_details.evolves_to.id
//         )
//     );

//     if let EvolutionMethod::LevelUp = evolution_details.method {
//         if evolution_level.is_empty() {
//             evolution_level.push_str(&format!(
//                 "| Base Pokemon | Level | Evolution |
//                             | :--: | :-- | :--: |
//                             "
//             ))
//         }
//         evolution_level.push_str(&entry);
//     }

//     if let EvolutionMethod::Item = evolution_details.method {
//         if evolution_item_iteraction.is_empty() {
//             evolution_item_iteraction.push_str(&format!(
//                 "| Base Pokemon | Item | Evolution |
//                                 | :--: | :-- | :--: |
//                                 "
//             ))
//         }
//         evolution_item_iteraction.push_str(&entry)
//     }

//     if let EvolutionMethod::Other = evolution_details.method {
//         if evolution_other.is_empty() {
//             evolution_other.push_str(&format!(
//                 "| Base Pokemon | Method | Evolution |
//                                 | :--: | :-- | :--: |
//                                 "
//             ))
//         }
//         evolution_other.push_str(&entry)
//     }
// }

// if !evolution_level.is_empty() {
//     let entry = format!("{}\n", evolution_level);
//     evolution_changes_markdown.push_str(&entry);
// }
// if !evolution_item_iteraction.is_empty() {
//     let entry = format!("{}\n", evolution_item_iteraction);
//     evolution_changes_markdown.push_str(&entry);
// }
// if !evolution_other.is_empty() {
//     let entry = format!("{}\n", evolution_other);
//     evolution_changes_markdown.push_str(&entry);
// }

// if evolution_changes_markdown.is_empty() {
//     if !evolution_page_exists {
//         return Ok("No Evolutions to generate".to_string());
//     }

//     match fs::remove_file(
//         base_path
//             .join(wiki_name)
//             .join("dist")
//             .join("docs")
//             .join("evolution_changes.md"),
//     ) {
//         Ok(file) => file,
//         Err(err) => {
//             println!("Failed to remove evolution changes filed: {}", err);
//         }
//     }
//     mkdocs_config
//         .nav
//         .as_sequence_mut()
//         .unwrap()
//         .remove(page_index);
//     match fs::write(
//         &mkdocs_yaml_file_path,
//         serde_yaml::to_string(&mkdocs_config).unwrap(),
//     ) {
//         Ok(file) => file,
//         Err(err) => return Err(format!("Failed to update mkdocs yaml file: {}", err)),
//     }
//     return Ok("No Evolution Changes to genereate. Evolution page removed".to_string());
// }

// evolution_changes_file
//     .write_all(format!("{}", evolution_changes_markdown).as_bytes())
//     .unwrap();

// if evolution_page_exists {
//     return Ok("Evolution Page Updated".to_string());
// }

// let mut evolution_changes = Mapping::new();
// evolution_changes.insert(
//     Value::String("Evolution Changes".to_string()),
//     Value::String("evolution_changes.md".to_string()),
// );

// mkdocs_config
//     .nav
//     .as_sequence_mut()
//     .unwrap()
//     .insert(1, Value::Mapping(evolution_changes));

// match fs::write(
//     mkdocs_yaml_file_path,
//     serde_yaml::to_string(&mkdocs_config).unwrap(),
// ) {
//     Ok(file) => file,
//     Err(err) => return Err(format!("Failed to update mkdocs yaml file: {}", err)),
// }

//     Ok("Evolution Page Generated".to_string())
// }

// fn get_level_item_method(evolution_details: &Evolution) -> String {
//     return match evolution_details.method {
//         EvolutionMethod::LevelUp => evolution_details.level.to_string(),
//         EvolutionMethod::Item => evolution_details.item.clone(),
//         EvolutionMethod::Other => evolution_details.other.clone(),
//         _ => "".to_string(),
//     };
// }
