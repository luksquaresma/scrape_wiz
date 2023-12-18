// use crate::utils::NamedEnum;

// use core::panic;
// use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::io::Read;

pub mod search_types {

    use crate::utils::{compare_to_typeid, compare_types, print_separator, NamedEnum};
    use strum::IntoEnumIterator;
    // use strum::IntoEnumIterator;
    use strum_macros::EnumIter;

    use core::panic;
    use serde::{Deserialize, Serialize};
    use std::any::TypeId;

    #[derive(Clone, Copy, Debug, Deserialize, EnumIter, PartialEq, Eq, Serialize)]
    pub enum PossibleSearchTypes {
        Google,
        LinkedIn,
        YouTube,
    }

    impl PossibleSearchTypes {
        pub fn get_search_url(&self, keyword: &String) -> String {
            match &self {
                PossibleSearchTypes::Google => {
                    format!(
                        "https://www.google.com/search?q={}",
                        keyword.replace(" ", "+")
                    )
                }
                PossibleSearchTypes::LinkedIn => {
                    todo!()
                }
                PossibleSearchTypes::YouTube => {
                    format!(
                        "https://www.youtube.com/results?search_query={}",
                        keyword.replace(" ", "+")
                    )
                }
            }
        }
    }

    impl NamedEnum for PossibleSearchTypes {
        fn get_name(&self) -> String {
            if let Some(p) = SEARCH_PAIRS.iter().find(|p| p.variant == *self) {
                p.name.to_string()
            } else {
                panic!("Impossible search! - Impossible search type error!")
            }
        }
        fn from_name(name: &String) -> Self {
            if let Some(p) = SEARCH_PAIRS.iter().find(|p| p.name == *name) {
                p.variant
            } else {
                panic!("Impossible search! - Impossible search name error!")
            }
        }
    }

    struct PssibleSearcheTypePair {
        variant: PossibleSearchTypes,
        name: &'static str,
    }

    const SEARCH_PAIRS: [PssibleSearcheTypePair; 3] = [
        PssibleSearcheTypePair {
            variant: PossibleSearchTypes::Google,
            name: "Google",
        },
        PssibleSearcheTypePair {
            variant: PossibleSearchTypes::LinkedIn,
            name: "LinkedIn",
        },
        PssibleSearcheTypePair {
            variant: PossibleSearchTypes::YouTube,
            name: "YouTube",
        },
    ];

    #[test]
    fn test() {
        // Testing possible search pairs
        // Itetares over every possible search pair and tests each possibility
        print_separator(0);
        println!("TESTING POSSIBLE SEARCH TYPES");

        {
            print_separator(1);
            println!("Testing types of name and variant");
            for search in SEARCH_PAIRS {
                print_separator(2);
                {
                    // Load the variable name, asserts its valid and prints it
                    let name = *&search.name;
                    assert!(compare_to_typeid(name, std::any::TypeId::of::<str>()));
                    println!("OK - Search type: {}", &name);
                };
                {
                    // Load the variable variant, asserts its valid and prints it
                    let variant = &search.variant;
                    assert!(compare_to_typeid(
                        variant,
                        std::any::TypeId::of::<PossibleSearchTypes>()
                    ));
                    println!("OK - Search variant: {:?}", &variant);
                };
            }
        };

        {
            print_separator(1);
            println!("Testing internal functions");
            for search in SEARCH_PAIRS {
                print_separator(2);
                {
                    // fn get_name
                    let name = PossibleSearchTypes::get_name(&search.variant);
                    assert!(name == search.name);
                    println!("OK - Gettin name {} from type: {:?}", name, search.variant);
                };

                {
                    // fn from_name
                    let variant = PossibleSearchTypes::from_name(&String::from(search.name));
                    assert!(variant == search.variant);

                    let name = variant.get_name();
                    assert!(name == search.name);

                    println!(
                        "OK - Getting type {:?} from name {}, then getting name {} from type {:?} \n",
                        variant, search.name, name, variant
                    );
                };
            }
        }
    }
}

pub mod search_pool {

    use serde::{Deserialize, Serialize};
    use std::collections::VecDeque;
    use std::fs::File;
    use std::io::Read;

    use crate::utils::NamedEnum;

    use super::search_types;

    // Raw search pool (Configs.keywords -> SearchConfig)
    #[derive(Debug, Deserialize, Serialize)]
    struct SearchConfig {
        keywords: Vec<String>,
        variants: Vec<String>,
    }

    impl SearchConfig {
        fn from_json(path: String) -> SearchConfig {
            // Open file as read-only
            let mut file = File::open(path).expect("Failed to open config file");

            // Read file into String
            let mut file_contents = String::new();
            file.read_to_string(&mut file_contents)
                .expect("Failed to read file contents");

            // Deserialize JSON file
            let config: SearchConfig =
                serde_json::from_str(&file_contents).expect("Failed to deserialize JSON");

            return config;
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Search {
        variant: search_types::PossibleSearchTypes,
        keyword: String,
        url: String,
    }

    impl Search {
        pub fn from_variant_name(variant_name: &String, keyword: &String) -> Search {
            let s_type = search_types::PossibleSearchTypes::from_name(variant_name);
            return Search {
                variant: s_type,
                keyword: keyword.clone(),
                url: s_type.get_search_url(keyword),
            };
        }

        pub fn vec_from_search_config(config: &SearchConfig) -> Vec<Search> {
            return config
                .variants
                .iter()
                .flat_map(|v| {
                    config
                        .keywords
                        .iter()
                        .map(|k| Search::from_variant_name(&v.to_string(), &k))
                        .collect::<Vec<Search>>()
                })
                .collect::<Vec<Search>>();
        }
    }

    struct SearchResults {
        serch_type_name: String,
        keyword: String,
        url: String,
        text_contents: String,
    }

    pub fn test() {
        const CONFIG_FILE_TEST: &'static str = "scraping/src/tests/config.json";

        println!("\n----------\nTESTING SEARCHES");
        {
            // SearchConfig struct testing
            println!("\n----------\nSearchConfig basic struct");
            let raw = SearchConfig {
                keywords: vec![
                    "test".to_string(),
                    "test 1".to_string(),
                    "test 1 2 3".to_string(),
                    "test 1 2 3 4 test".to_string(),
                ],
                variants: vec!["Google".to_string()],
            };
            println!("Basic struct: \n{:#?}", raw);
        };
        {
            // SearchConfig struct loading from JSON
            println!("\n----------\nSearchConfig loading from JSON");
            let from_j = SearchConfig::from_json(CONFIG_FILE_TEST.to_string());
            println!("From JSON: \n{:#?}", from_j);
        };
        {};
    }

    // // Read the configuration from the JSON file
    // let searches = SearchConfig::from_json(String::from(CONFIGS.search_config_path));

    // println!("Target: \n {:#?}", searches);

    // let target_urls: Vec<String> = searches.get_search_urls();

    // // println!("Target URLS: \n {:#?}", target_urls);
}
