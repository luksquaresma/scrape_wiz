pub mod search;
pub mod utils;

use chrono::expect;
use core::panic;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

use crate::utils::NamedEnum;

#[derive(Debug, Deserialize, Serialize)]
struct Configs {
    search_config_path: &'static str,
}

const CONFIGS: Configs = Configs {
    search_config_path: "./config.json",
};

#[derive(Debug, Deserialize, Serialize)]
struct RawSearchPool {
    keywords: Vec<String>,
    modes: Vec<String>
}

impl RawSearchPool {
    fn from_json(path: String) -> RawSearchPool {
        let file_path = path;

        // Open file in read-only mode
        let mut file = File::open(file_path).expect("Failed to open config file");

        // Read file contents into String
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)
            .expect("Failed to read file contents");

        // Deserialize JSON file
        let config: RawSearchPool =
            serde_json::from_str(&file_contents).expect("Failed to deserialize JSON");

        return config;
    }
}

// struct SearchJob {
//     keyword: String;
//     mode:
// }

// struct SearchPool {
//     keywords:
//     modes:
// }






//         // Search urls by type
//         fn get_search_urls_by_type(&self, search_type:String) -> Vec<String>{
//             match search_type {
    
//                 // Google search
//                 String::from("google") => self.keywords.iter().map(
//                     |search|
//                     format!("https://www.google.com/search?q={}", search.replace(" ", "+"))
//                     ).collect()
    
//                 // Panic with no compatible source
//                 //  {
//                 //     panic!("No compatible searches with type {search_type}")
//                 // }
//                 }
//             }

// // Search urls by type
// fn get_search_urls_by_type(&self, search_type:String) -> Vec<String>{
//     match search_type {

//         // Google search
//         String::from("google") => self.keywords.iter().map(
//             |search|
//             format!("https://www.google.com/search?q={}", search.replace(" ", "+"))
//             ).collect()

//         // Panic with no compatible source
//         //  {
//         //     panic!("No compatible searches with type {search_type}")
//         // }
//     }
//     }

//     fn get_search_urls(&self) {

//         // Gettil urls for all modes
//         self.modes.iter().flat_map(
//             |mode|
//             self.get_search_urls_by_type(mode)
//         ).collect()
//     }


fn main() {

    // Read the configuration from the JSON file
    let searches = RawSearchPool::from_json(String::from(CONFIGS.search_config_path));

    println!("Target: \n {:#?}", searches);

    let target_urls: Vec<String> = searches.get_search_urls();

    println!("Target URLS: \n {:#?}", target_urls);

    // Send a GET request to the URL
    let response = get(&target_urls[0]).expect("Failed to send request");

    // println!("{:#?}",response.text().expect("Not working"));
    // println!("{:#?}",response.url().query());

    let query = match response.url().query() {
        Some(val) => match String::from(val).strip_prefix("q=") {
            Some(val) => val.replace("+", " "),
            None => String::from("No q= in the querys"),
        },
        None => String::from("Unavailable"),
    };

    // let query = match response.url().query() {
    //     Some(val) => {
    //         match val.strip_prefix("q=") {
    //             Some(val) => val.replace("+", " ").clone() as &str,
    //             None => "No q= in the query"
    //         }
    //     },
    //     None => "Unavailable",
    // };

    println!("{:#?}", query);

    // // Check if the request was successful (status code 200)
    // if response.status().is_success() {
    //     // Parse the HTML content of the page
    //     let body = response.text().expect("Failed to parse HTML");
    //     let document = Document::from_read(body.as_bytes()).expect("Failed to parse HTML");

    //     // Extract and print all text content from paragraph (p) elements
    //     for node in document.find(Name("p")) {
    //         // Extract text from the node and handle the Option<String>
    //         let text = node.text().unwrap_or_default();
    //         println!("{}", text);
    //     }
    // } else {
    //     println!("Request failed with status code: {}", response.status());
    // }
}
