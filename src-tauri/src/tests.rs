#[cfg(test)]
pub mod tests {
    use std::path::PathBuf;

    use crate::page_generators::game_routes::generate_route_pages;

    #[test]
    fn test_generate_route_pages() {
        let base_path =
            PathBuf::from("/Users/akeemallen/Library/Application Support/com.wikigen.dev/");

        let result = generate_route_pages("aspertia", base_path).unwrap();
        println!("{}", result);
        assert_eq!(3, 3)
    }
}
