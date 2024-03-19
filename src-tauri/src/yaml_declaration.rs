use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct MKDocsConfig {
    site_name: String,
    site_url: String,
    site_description: String,
    site_author: String,
    repo_url: String,
    theme: Theme,
    nav: Vec<HashMap<String, Navigation>>,
    plugins: Vec<Plugin>,
    markdown_extensions: Vec<MarkdownExtension>,
}

#[derive(Serialize, Deserialize)]
struct Theme {
    name: String,
    features: Vec<String>,
    favicon: String,
    palette: [Palette; 2],
}

#[derive(Serialize, Deserialize)]
struct Palette {
    media: String,
    primary: String,
    scheme: String,
    toggle: Toggle,
}

#[derive(Serialize, Deserialize)]
struct Toggle {
    icon: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Plugin {
    search: Search,
}

#[derive(Serialize, Deserialize)]
struct Search {
    lang: String,
}

#[derive(Deserialize, Clone)]
enum Navigation {
    String(String),
    Array(Vec<Navigation>),
    Map(HashMap<String, Navigation>),
}

impl Serialize for Navigation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Navigation::String(s) => serializer.serialize_str(s),
            Navigation::Array(a) => {
                let mut seq = serializer.serialize_seq(Some(a.len()))?;
                for e in a {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            Navigation::Map(m) => {
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

#[derive(Deserialize, Clone)]
enum MarkdownExtension {
    String(String),
    KeyValue {
        key: String,
        value: HashMap<String, bool>,
    },
}

impl Serialize for MarkdownExtension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MarkdownExtension::String(s) => serializer.serialize_str(s),
            MarkdownExtension::KeyValue { key, value } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(key, value)?;
                map.end()
            }
        }
    }
}

// Since I'm new to rust, almost everything in here is likely poorly done, but it works for now.
pub fn get_yaml(
    site_name: &str,
    site_url: &str,
    site_description: &str,
    site_author: &str,
    repo_url: &str,
) -> MKDocsConfig {
    let mut custom_checkbox = HashMap::new();
    custom_checkbox.insert("custom_checkbox".to_string(), true);

    let mut alternate_style = HashMap::new();
    alternate_style.insert("alternate_style".to_string(), true);

    let mkdocs_config = MKDocsConfig {
        site_name: site_name.to_string(),
        site_url: site_url.to_string(),
        site_description: site_description.to_string(),
        site_author: site_author.to_string(),
        repo_url: repo_url.to_string(),
        theme: Theme {
            name: "material".to_string(),
            features: vec![
                "content.tabls.links".to_string(),
                "content.tooltips".to_string(),
            ],
            favicon: "img/items/poke-ball.png".to_string(),
            palette: [
                Palette {
                    media: "(prefers-color-scheme: light)".to_string(),
                    primary: "black".to_string(),
                    scheme: "default".to_string(),
                    toggle: Toggle {
                        icon: "material/eye-outline".to_string(),
                        name: "Switch to dark mode".to_string(),
                    },
                },
                Palette {
                    media: "(prefers-color-scheme: dark)".to_string(),
                    primary: "black".to_string(),
                    scheme: "default".to_string(),
                    toggle: Toggle {
                        icon: "material/eye".to_string(),
                        name: "Switch to light mode".to_string(),
                    },
                },
            ],
        },
        nav: vec![
            [(
                "Home".to_string(),
                Navigation::String("index.md".to_string()),
            )]
            .iter()
            .cloned()
            .collect(),
            {
                let mut pokemon_map = HashMap::new();
                let mut specific_changes = HashMap::new();
                let mut test_pokemon = HashMap::new();
                test_pokemon.insert(
                    "Test Pokemon".to_string(),
                    Navigation::String("pokemon/test_pokemon.md".to_string()),
                );
                specific_changes.insert(
                    "Specific Changes".to_string(),
                    Navigation::Array(vec![Navigation::Map(test_pokemon)]),
                );
                pokemon_map.insert(
                    "Pokemon".to_string(),
                    Navigation::Array(vec![Navigation::Map(specific_changes)]), // Navigation::String("pokemon/test_pokemon.md".to_string()),
                );
                pokemon_map
            },
            {
                let mut routes_map = HashMap::new();
                let mut test_route = HashMap::new();
                let mut wild_encounters = HashMap::new();
                wild_encounters.insert(
                    "Wild Encounters".to_string(),
                    Navigation::String("routes/Test_route/wild_encounters.md".to_string()),
                );
                test_route.insert(
                    "Test Route".to_string(),
                    Navigation::Array(vec![Navigation::Map(wild_encounters)]),
                );
                routes_map.insert(
                    "Routes".to_string(),
                    Navigation::Array(vec![Navigation::Map(test_route)]),
                );
                routes_map
            },
        ],
        plugins: vec![Plugin {
            search: Search {
                lang: "en".to_string(),
            },
        }],
        markdown_extensions: [
            MarkdownExtension::String("admonition".to_string()),
            MarkdownExtension::String("abbr".to_string()),
            MarkdownExtension::String("attr_list".to_string()),
            MarkdownExtension::String("pymdownx.snippets".to_string()),
            MarkdownExtension::String("pymdownx.superfences".to_string()),
            MarkdownExtension::KeyValue {
                key: "pymdownx.tasklist".to_string(),
                value: custom_checkbox,
            },
            MarkdownExtension::KeyValue {
                key: "pymdownx.tabbed".to_string(),
                value: alternate_style,
            },
        ]
        .to_vec(),
    };

    return mkdocs_config;
}
