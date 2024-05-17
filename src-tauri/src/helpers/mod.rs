pub mod capitalize;
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
