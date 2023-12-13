use std::fs::File;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use select::document::Document;
use select::predicate::Name;
use reqwest::blocking::Client;
// blocking::Client;
use chrono::prelude::*;

#[derive(Debug, Deserialize)]
struct Config {
    keywords: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ScrapedData {
    date_time: String,
    keyword_searched: String,
    url_opened: String,
    main_url_part: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the configuration from the JSON file
    let config_content = std::fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&config_content)?;

    // Initialize an empty vector to store scraped data
    let mut scraped_data_vec: Vec<ScrapedData> = Vec::new();

    for keyword in &config.keywords {
        // Define the target URL based on the provided keyword
        let target_url = format!("https://www.google.com/search?q={}", keyword.replace(" ", "+"));

        // Make an HTTP request to the target URL
        let response = Client::new().get(&target_url).send()?;
        let html_content = response.text()?;

        // Extract relevant information from the HTML
        let document = Document::from_read(html_content.as_bytes())?;
        if let Some(first_result) = document.find(Name("a")).next() {
            let url_opened = first_result.attr("href").unwrap_or_default();
            let main_url_part = url_opened.split('/').next().unwrap_or_default();

            // Prepare data to be added to the vector
            let scraped_data = ScrapedData {
                date_time: Utc::now().to_rfc3339(),
                keyword_searched: keyword.clone(),
                url_opened: url_opened.to_string(),
                main_url_part: main_url_part.to_string(),
            };

            // Add the scraped data to the vector
            scraped_data_vec.push(scraped_data);
        } else {
            println!("No results found for keyword: {}", keyword);
        }
    }

    // Write all scraped data to a single JSON file
    let mut output_file = File::create("output.json")?;
    let output_json = serde_json::to_string_pretty(&scraped_data_vec)?;
    writeln!(output_file, "{}", output_json)?;
    println!("Scraping completed successfully!");

    Ok(())
}
