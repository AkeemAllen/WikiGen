use std::{fs, io, path::Path};

pub mod matchups;
pub mod mkdocs_process;

pub fn get_pokemon_dex_formatted_name(dex_number: usize) -> String {
    let mut pokedex_markdown_file_name = format!("00{}", dex_number);
    if dex_number >= 10 {
        pokedex_markdown_file_name = format!("0{}", dex_number);
    }
    if dex_number >= 100 {
        pokedex_markdown_file_name = format!("{}", dex_number);
    }
    return pokedex_markdown_file_name;
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Copy files from source to destination recursively.
pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
