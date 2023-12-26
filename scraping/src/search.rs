// use crate::utils::NamedEnum;

// use core::panic;
// use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::io::Read;

pub mod search_types {

    use core::panic;
    use serde::{Deserialize, Serialize};
    use strum_macros::EnumIter;

    // Defines all possible search types
    #[derive(Clone, Copy, Debug, Deserialize, EnumIter, Eq, PartialEq, Serialize)]
    pub enum PossibleSearchTypes {
        Google,
        // LinkedIn,
        YouTube,
    }

    impl PossibleSearchTypes {
        pub fn get_name(&self) -> String {
            if let Some(p) = SEARCH_PAIRS.iter().find(|p| p.variant == *self) {
                p.name.to_string()
            } else {
                panic!("Impossible search! - Impossible search type error!")
            }
        }

        pub fn from_name(name: &String) -> Self {
            if let Some(p) = SEARCH_PAIRS.iter().find(|p| p.name == *name) {
                p.variant
            } else {
                panic!("Impossible search! - Impossible search nae error!")
            }
        }

        pub fn get_search_url(&self, keyword: &String) -> String {
            match &self {
                PossibleSearchTypes::Google => {
                    format!(
                        "https://www.google.com/search?q={}",
                        keyword.replace(" ", "+")
                    )
                }
                // PossibleSearchTypes::LinkedIn => {
                //     todo!()
                // },
                PossibleSearchTypes::YouTube => {
                    format!(
                        "https://www.youtube.com/results?search_query={}",
                        keyword.replace(" ", "+")
                    )
                }
            }
        }
    }

    // Ties a speccific search type with its name
    pub struct PssibleSearcheTypePair {
        pub variant: PossibleSearchTypes,
        pub name: &'static str,
    }

    // Hard coded definition of the pairs Search-Name
    pub(crate) const SEARCH_PAIRS: [PssibleSearcheTypePair; 2] = [
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
}

pub mod search_pool {

    use reqwest::Response;
    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::Read;

    use crate::utils::REQUEST_MAX_TRIES;

    use super::search_types;

    // Read defines the required inputs for creatong search jobs
    #[derive(Debug, Deserialize, Serialize)]
    pub struct SearchConfig {
        pub keywords: Vec<String>, // theese have to strings for now
        pub variants: Vec<String>, // theese have to strings for now
    }
    impl SearchConfig {
        pub fn from_json(path: String) -> SearchConfig {
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
    #[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub struct Search {
        pub variant: search_types::PossibleSearchTypes,
        pub keyword: String,
        pub url: String,
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

        // pub fn get_text_output(&self, trys: &u32) -> Option<String> {
        //     let mut t: u32 = trys.clone();
        //     while t > 0 {
        //         if let Ok(raw) = reqwest::blocking::get(&self.url) {
        //             if let Ok(resp) = raw.text() {
        //                 return Some(resp);
        //             } else {
        //                 t -= 1;
        //             }
        //         } else {
        //             t -= 1;
        //         }
        //     }
        //     return None;
        // }

        // pub fn perform(&self) -> Option<SearchResult> {
        //     if let Some(resp) = self.get_text_output(&REQUEST_MAX_TRIES) {
        //         return Some(
        //             SearchResult {
        //                 variant: self.variant.clone(),
        //                 keyword: self.keyword.clone(),
        //                 url: self.url.clone(),
        //                 output: resp
        //             });
        //     } else {
        //         None
        //     }
        // }
    }

    // Defines a serach result
    pub struct SearchResult {
        pub variant: search_types::PossibleSearchTypes,
        pub keyword: String,
        pub url: String,
        pub output: String,
    }
}

#[cfg(test)]
mod tests {
    use super::{search_pool, search_types};
    use crate::utils::{
        compare_to_typeid, print_separator, CONFIG_FILE_TEST, TESTING_SEARCH_KEYWORDS,
    };
    use strum::IntoEnumIterator;

    #[test]
    fn test_search_types() {
        // Itetares over every possible search pair and tests each possibility
        print_separator(0);
        println!("TESTING POSSIBLE SEARCH TYPES");

        {
            print_separator(1);
            println!("Testing types of name and variant:");
            for search in search_types::SEARCH_PAIRS {
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
                        std::any::TypeId::of::<search_types::PossibleSearchTypes>()
                    ));
                    println!("OK - Type of search variant {:?}", &variant);
                };
            }
        };

        {
            print_separator(1);
            println!("Testing internal functions:");
            println!("Testing searches used: \n{:#?}", TESTING_SEARCH_KEYWORDS);

            for search in search_types::SEARCH_PAIRS {
                print_separator(2);
                {
                    // fn get_name
                    let name = search_types::PossibleSearchTypes::get_name(&search.variant);
                    assert!(name == search.name);
                    println!("OK - Getting name {} from type {:?}", name, search.variant);
                };

                {
                    // fn from_name
                    let variant =
                        search_types::PossibleSearchTypes::from_name(&String::from(search.name));
                    assert!(variant == search.variant);

                    let name = variant.get_name();
                    assert!(name == search.name);

                    println!("OK - Getting type {:?} from name {}", variant, search.name);
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
                    println!("OK - Search urls \n{:#?}", urls);
                };
            }
        }
    }

    #[test]
    fn test_search_pool() {
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
                let raw = search_pool::SearchConfig {
                    keywords: TESTING_SEARCH_KEYWORDS
                        .iter()
                        .map(|k| k.to_string())
                        .collect::<Vec<String>>(),
                    variants: search_types::PossibleSearchTypes::iter()
                        .map(|v| v.get_name())
                        .collect::<Vec<String>>(),
                };
                println!("Basic declaration: \n{:#?}", raw);
            };
            {
                // Loading from JSON - running tests
                print_separator(2);
                let from_j = search_pool::SearchConfig::from_json(CONFIG_FILE_TEST.to_string());
                println!("From JSON: \n{:#?}", from_j);
            };
        };

        {
            // struct Search
            print_separator(1);
            println!("Struct Search");
            {
                // Raw type check
                print_separator(2);
                for v in search_types::PossibleSearchTypes::iter() {
                    for &k in TESTING_SEARCH_KEYWORDS.iter() {
                        assert_eq!(
                            search_pool::Search {
                                variant: v,
                                keyword: k.to_string(),
                                url: v.get_search_url(&k.to_string())
                            },
                            search_pool::Search::from_variant_name(&v.get_name(), &k.to_string())
                        )
                    }
                }
                println!("OK - Constructor from variant name checking.");
            };

            {
                // Composed construction
                print_separator(2);

                let searches_raw = search_types::PossibleSearchTypes::iter()
                    .flat_map(|v| {
                        TESTING_SEARCH_KEYWORDS
                            .iter()
                            .map(|k| search_pool::Search {
                                variant: v.clone(),
                                keyword: k.to_string(),
                                url: v.get_search_url(&k.to_string()),
                            })
                            .collect::<Vec<search_pool::Search>>()
                    })
                    .collect::<Vec<search_pool::Search>>();

                let searches_manual = search_types::PossibleSearchTypes::iter()
                    .flat_map(|v| {
                        TESTING_SEARCH_KEYWORDS
                            .iter()
                            .map(|k| {
                                search_pool::Search::from_variant_name(
                                    &v.get_name(),
                                    &k.to_string(),
                                )
                            })
                            .collect::<Vec<search_pool::Search>>()
                    })
                    .collect::<Vec<search_pool::Search>>();

                let searches_automatic = search_pool::Search::vec_from_search_config(
                    &search_pool::SearchConfig::from_json(CONFIG_FILE_TEST.to_string()),
                );

                assert!(searches_raw == searches_manual);

                // println!("searches_raw {:#?}", searches_raw);
                // println!("searches_automatic {:#?}", searches_automatic);

                assert!(searches_raw == searches_automatic);
                println!("OK - Composed construction.");

                print_separator(2);
                println!("Tested searches: \n{:#?}", searches_automatic);
            }
        }
    }
}
