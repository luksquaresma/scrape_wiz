// use crate::utils::NamedEnum;

// use core::panic;
// use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::io::Read;

pub mod search_types {

    use crate::utils::{compare_to_typeid, print_separator, NamedEnum, TESTING_SEARCH_KEYWORDS};
    use strum::IntoEnumIterator;
    // use strum::IntoEnumIterator;
    use strum_macros::EnumIter;

    use core::panic;
    use serde::{Deserialize, Serialize};
    use std::any::TypeId;

    // Defines all possible search types
    #[derive(Clone, Copy, Debug, Deserialize, EnumIter, PartialEq, Eq, Serialize)]
    pub enum PossibleSearchTypes {
        Google,
        // LinkedIn,
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
                },
                // PossibleSearchTypes::LinkedIn => {
                //     todo!()
                // },
                PossibleSearchTypes::YouTube => {
                    format!(
                        "https://www.youtube.com/results?search_query={}",
                        keyword.replace(" ", "+")
                    )
                },
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

    // Ties a speccific search type with its name
    struct PssibleSearcheTypePair {
        variant: PossibleSearchTypes,
        name: &'static str,
    }

    // Hard coded definition of the pairs Search-Name
    const SEARCH_PAIRS: [PssibleSearcheTypePair; 2] = [
        PssibleSearcheTypePair {
            variant: PossibleSearchTypes::Google,
            name: "Google",
        },
        // PssibleSearcheTypePair {
        //     variant: PossibleSearchTypes::LinkedIn,
        //     name: "LinkedIn",
        // },
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
            println!("Testing types of name and variant:");
            for search in SEARCH_PAIRS {
                print_separator(2);
                {
                    // Load the variable name, asserts its valid and prints it
                    let name = *&search.name;
                    assert!(compare_to_typeid(name, std::any::TypeId::of::<str>()));
                    println!("OK - Type of search named {}", &name);
                };
                {
                    // Load the variable variant, asserts its valid and prints it
                    let variant = &search.variant;
                    assert!(compare_to_typeid(
                        variant,
                        std::any::TypeId::of::<PossibleSearchTypes>()
                    ));
                    println!("OK - Type of search variant {:?}", &variant);
                };
            }
        };

        {
            print_separator(1);
            println!("Testing internal functions:");
            println!("Testing searches used: {:#?}", TESTING_SEARCH_KEYWORDS);

            for search in SEARCH_PAIRS {
                print_separator(2);
                {
                    // fn get_name
                    let name = PossibleSearchTypes::get_name(&search.variant);
                    assert!(name == search.name);
                    println!("OK - Getting name {} from type {:?}", name, search.variant);
                };

                {
                    // fn from_name
                    let variant = PossibleSearchTypes::from_name(&String::from(search.name));
                    assert!(variant == search.variant);

                    let name = variant.get_name();
                    assert!(name == search.name);

                    println!(
                        "OK - Getting type {:?} from name {}", variant, search.name
                    );
                };

                {
                    // fn get_search_url -> Only a running test, not a double check yet
                    let urls = TESTING_SEARCH_KEYWORDS
                        .iter()
                        .map(|keyword| {
                            let url = search.variant.get_search_url(&keyword.to_string());
                            assert!(compare_to_typeid(&url, std::any::TypeId::of::<String>()));
                            return url;
                        })
                        .collect::<Vec<String>>();
                    println!("OK - Search urls {:#?}", urls);
                };
            }
        }
    }
}

pub mod search_pool {
    
    use serde::{Deserialize, Serialize};
    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;
    use std::collections::VecDeque;
    use std::fs::File;
    use std::io::Read;

    use crate::{utils::{NamedEnum, print_separator, TESTING_SEARCH_KEYWORDS, CONFIG_FILE_TEST}, search::search_types::PossibleSearchTypes};

    use super::search_types;

    // Read defines the required inputs for creatong search jobs
    #[derive(Debug, Deserialize, Serialize)]
    struct SearchConfig {
        keywords: Vec<String>, // theese have to strings for now
        variants: Vec<String>, // theese have to strings for now
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


    // Defines a search job
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


    // Defines a serach result
    struct SearchResults {
        serch_type_name: String,
        keyword: String,
        url: String,
        text_contents: String,
    }

    #[test]
    fn test() {
        // Testing possible search pairs
        // Itetares over every possible search pair and tests each possibility
        print_separator(0);
        println!("TESTING SEARCH POOL");
        println!("Configuration file used: {}", CONFIG_FILE_TEST);

        {
            // struct SearchConfig
            print_separator(1);
            println!("Struct SearchConfig");
            {
                // Simple declarations - running tests
                print_separator(2);
                let raw = SearchConfig {
                    keywords: TESTING_SEARCH_KEYWORDS.iter().map(|k| k.to_string()).collect::<Vec<String>>(),
                    variants: PossibleSearchTypes::iter().map(|v| v.get_name()).collect::<Vec<String>>(),
                };
                println!("Basic declaration: \n{:#?}", raw);
            };
            {
                // Loading from JSON - running tests
                print_separator(2);
                let from_j = SearchConfig::from_json(CONFIG_FILE_TEST.to_string());
                println!("From JSON: {:#?}", from_j);
            };
        };

        {
            // struct Search
            print_separator(1);
            println!("Struct SearchConfig");

        }
    }

    // // Read the configuration from the JSON file
    // let searches = SearchConfig::from_json(String::from(CONFIGS.search_config_path));

    // println!("Target: \n {:#?}", searches);

    // let target_urls: Vec<String> = searches.get_search_urls();

    // // println!("Target URLS: \n {:#?}", target_urls);
}
