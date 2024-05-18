pub mod copy_recursively;
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
