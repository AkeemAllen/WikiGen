use serde::de::Visitor;
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MKDocsConfig {
    pub site_name: String,
    pub site_url: String,
    pub site_description: String,
    pub site_author: String,
    pub repo_url: String,
    pub theme: Theme,
    pub nav: Vec<HashMap<String, Navigation>>,
    pub plugins: Vec<Plugin>,
    pub markdown_extensions: Vec<MarkdownExtension>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub features: Vec<String>,
    pub favicon: String,
    pub palette: [Palette; 2],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Palette {
    pub media: String,
    pub primary: String,
    pub scheme: String,
    pub toggle: Toggle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Toggle {
    pub icon: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    pub search: Search,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Search {
    pub lang: String,
}

#[derive(Debug, Clone)]
pub enum Navigation {
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

impl<'de> Deserialize<'de> for Navigation {
    fn deserialize<D>(deserializer: D) -> Result<Navigation, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum NavHelper {
            String(String),
            Array(Vec<Navigation>),
            Map(HashMap<String, Navigation>),
        }

        let nav_helper = NavHelper::deserialize(deserializer)?;

        match nav_helper {
            NavHelper::String(s) => Ok(Navigation::String(s)),
            NavHelper::Array(a) => Ok(Navigation::Array(a)),
            NavHelper::Map(m) => Ok(Navigation::Map(m)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MarkdownExtension {
    String(String),
    /*
    I originally used KeyValue map here to represent PymdownxTaskList and PymdownxTabbed.
    Ideally, this made the program more extensible.
    However, due to my skill issues with deserialization,
    I opted to use these structs instead.

    This makes the work easier, however, this will break if I need to add more extensions
    or a user adds their own extensions to the mkdocs.yml file.
     */
    PymdownxTaskList(PymdownxTaskList),
    PymdownxTabbed(PymdownxTabbed),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PymdownxTaskList {
    pub custom_checkbox: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PymdownxTabbed {
    pub alternate_style: bool,
}

impl Serialize for MarkdownExtension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MarkdownExtension::String(s) => serializer.serialize_str(s),
            MarkdownExtension::PymdownxTabbed(p) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("pymdownx.tabbed", &p)?;
                map.end()
            }
            MarkdownExtension::PymdownxTaskList(p) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("pymdownx.tasklist", &p)?;
                map.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for MarkdownExtension {
    fn deserialize<D>(deserializer: D) -> Result<MarkdownExtension, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MarkdownExtensionVisitor;

        impl<'de> Visitor<'de> for MarkdownExtensionVisitor {
            type Value = MarkdownExtension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string or key-value map")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(MarkdownExtension::String(value.to_owned()))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(MarkdownExtension::String(value))
            }

            fn visit_map<A>(self, mut map: A) -> Result<MarkdownExtension, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let key: String = map
                    .next_key()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                match key.as_str() {
                    "pymdownx.tasklist" => {
                        let p: PymdownxTaskList = map.next_value()?;
                        Ok(MarkdownExtension::PymdownxTaskList(p))
                    }
                    "pymdownx.tabbed" => {
                        let p: PymdownxTabbed = map.next_value()?;
                        Ok(MarkdownExtension::PymdownxTabbed(p))
                    }
                    _ => {
                        let s: String = map.next_value()?;
                        Ok(MarkdownExtension::String(s))
                    }
                }
            }
        }

        deserializer.deserialize_any(MarkdownExtensionVisitor)
    }
}
