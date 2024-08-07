#[cfg(test)]
pub mod tests {
    use std::path::PathBuf;

    use crate::page_generators::pokemon_pages::generate_pokemon_page;

    // #[test]
    // fn test_create_dir_all() {
    //     let base_path =
    //         PathBuf::from("/Users/akeemallen/Library/Application Support/com.wikigen.dev/test");
    //     let base_path_2 =
    //         PathBuf::from("/Users/akeemallen/Library/Application Support/com.wikigen.dev/Test");

    //     match std::fs::create_dir_all(&base_path) {
    //         Ok(_) => {
    //             println!("Lower case Directory created");
    //         }
    //         Err(err) => {
    //             println!("Error creating lower case directory: {}", err);
    //         }
    //     };
    //     let path_1_exists = base_path.try_exists().unwrap();
    //     println!("{:?}", path_1_exists);

    //     match std::fs::create_dir_all(&base_path_2) {
    //         Ok(_) => {
    //             println!("Higher case Directory created");
    //         }
    //         Err(err) => {
    //             println!("Error creating higher case directory: {}", err);
    //         }
    //     };

    //     let path_2_exists = base_path_2.try_exists().unwrap();
    //     println!("{:?}", path_2_exists);
    // }

    // #[test]
    // fn test_generate_pokemon_page(){
    //     let base_path =
    //                 PathBuf::from("/Users/akeemallen/Library/Application Support/com.wikigen.dev/test-wiki");

    //     let result = generate_pokemon_page(&base_path, &pokemon);
    // }
}
