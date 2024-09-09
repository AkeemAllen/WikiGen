use std::{
    collections::HashMap,
    fs::{self, read_to_string, File},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

pub type Wikis = HashMap<String, Wiki>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wiki {
    name: String,
    description: String,
    author: String,
    site_name: String,
    site_url: String,
    repo_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    deployment_url: String,
}

pub fn run_file_migrations(base_path: &PathBuf, resource_path: &PathBuf) -> Result<(), String> {
    let wiki_json_file_path = base_path.join("wikis.json");
    let wikis_file = match File::open(&wiki_json_file_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to read wikis file: {}", err)),
    };
    let wikis: Wikis = match serde_json::from_reader(wikis_file) {
        Ok(wikis) => wikis,
        Err(err) => return Err(format!("Failed to parse wikis file: {}", err)),
    };

    for (wiki_name, wiki) in wikis.iter() {
        let extra_css_file_path = base_path
            .join(wiki_name)
            .join("dist")
            .join("docs")
            .join("stylesheets")
            .join("extra.css");
        let extra_css = match read_to_string(&extra_css_file_path) {
            Ok(lines) => lines,
            Err(err) => return Err(format!("Failed to read extra css file: {}", err)),
        };

        let original_css_file_path = resource_path
            .join("resources")
            .join("generator_assets")
            .join("stylesheets")
            .join("extra.css");
        let original_css = match read_to_string(original_css_file_path) {
            Ok(lines) => lines,
            Err(err) => return Err(format!("Failed to read original css file: {}", err)),
        };

        if extra_css.contains(".pokemon-attribute-container") {
            continue;
        }

        match fs::write(&extra_css_file_path, original_css) {
            Ok(_) => {}
            Err(err) => return Err(format!("Failed to write to extra css file: {}", err)),
        }
    }
    Ok(())
}
