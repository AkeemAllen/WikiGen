pub const TRUE: i32 = 1;
pub const FALSE: i32 = 0;

use std::{fs, io, path::Path};

pub mod json_conversion;
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

pub fn capitalize_and_remove_hyphens(input: &str) -> String {
    input
        .split('-') // Split the string by hyphens
        .map(|word| {
            // Capitalize the first letter and make the rest lowercase
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<String>>() // Collect the words into a vector
        .join(" ") // Join the words with spaces
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

pub fn round_up_to_nearest_100(num: u32) -> u32 {
    if num % 100 == 0 {
        num
    } else {
        ((num / 100) + 1) * 100
    }
}
