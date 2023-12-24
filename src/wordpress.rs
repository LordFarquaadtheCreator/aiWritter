use dotenv::dotenv;
use std::{env, error::Error};
// use colored::*;
use reqwest;
use base64;
use urlencoding::encode;
use base64::{Engine as _, engine::general_purpose};

#[tokio::main]
async fn find_tag(tag: String) -> Result<i64, Box<dyn Error>> {
    dotenv().expect("Failed to read .env file");
    let password: String = env::var("PASSWORD").unwrap().to_string(); 

    let url = format!("https://rightondigital.com/wp-json/wp/v2/tags?search={}", encode(&tag));
    let creds = general_purpose::STANDARD.encode(format!("fahadfaruqi1@gmail.com:{}", password));

    // query wordpress for all avaliable tags
    let wordpress: reqwest::Client = reqwest::Client::new();
    let existing_tag_response: reqwest::Response = wordpress.get(&url)
        .header("Authorization", format!("Basic {}", &creds))
        .send()
        .await?;
    let existing_tag: String = existing_tag_response.text().await?;
    let existing_tags: serde_json::Value = serde_json::from_str(&existing_tag)?;

    if existing_tags.as_array().map_or(true, |v| v.is_empty()) { 
        // array is empty meaning we must create a new tag
        let result = create_tag(&wordpress, &url, &creds, &tag).await;
        match result {
            Ok(result) => Ok(result),
            Err(err) => Err(err),
        }
    } else { // array not empty so return tag's ID
        match existing_tags[0]["id"].as_i64() {
            Some(id) => Ok(id),
            None => Err("Invalid ID".into()),
        }
    }
}

async fn create_tag(wordpress: &reqwest::Client, url: &str, creds: &str, tag: &str,) -> Result<i64, Box<dyn std::error::Error>> {
    let body = format!("{{\"name\":\"{}\"}}", tag);
    let create_tag_response: reqwest::Response = wordpress.post(url)
        .header("Authorization", format!("Basic {}", creds))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    let create_tag_response: String = create_tag_response.text().await?;
    let create_tag_response_json: serde_json::Value = serde_json::from_str(&create_tag_response)?;

    match create_tag_response_json["id"].as_i64() {
        Some(id) => Ok(id),
        None => Err(create_tag_response.to_string().into()),
    }
}

pub fn wordpress() {
    let result = find_tag("testingfahad123457".to_string());
    match result {
        Ok(result) => println!("{}", result),
        Err(err) => println!("Error: {}", err),
    }
}