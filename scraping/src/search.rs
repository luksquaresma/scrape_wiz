// use crate::utils::NamedEnum;

// use core::panic;
// use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::io::Read;

pub mod search_types {

    use crate::utils::NamedEnum;

    use core::panic;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum PossibleSearchTypes {
        Google,
        LinkedIn,
        YouTube,
    }

    impl NamedEnum for PossibleSearchTypes {
        fn get_name(&self) -> &'static str {
            if let Some(p) = SEARCH_PAIRS.iter().find(|p| p.search_type == *self) {
                p.search_name
            } else {
                panic!("Impossible search! - Impossible search type error!")
            }
        }
        fn from_name(name: &'static str) -> Self {
            if let Some(p) = SEARCH_PAIRS.iter().find(|p| p.search_name == name) {
                p.search_type
            } else {
                panic!("Impossible search! - Impossible search name error!")
            }
        }
    }

    struct PssibleSearcheTypePair {
        search_type: PossibleSearchTypes,
        search_name: &'static str,
    }

    const SEARCH_PAIRS: [PssibleSearcheTypePair; 3] = [
        PssibleSearcheTypePair {
            search_type: PossibleSearchTypes::Google,
            search_name: "Google",
        },
        PssibleSearcheTypePair {
            search_type: PossibleSearchTypes::LinkedIn,
            search_name: "LinkedIn",
        },
        PssibleSearcheTypePair {
            search_type: PossibleSearchTypes::YouTube,
            search_name: "YouTube",
        },
    ];
}

pub mod search_pool {

}