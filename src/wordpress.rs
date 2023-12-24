use dotenv::dotenv;
use std::{env, error::Error};
// use colored::*;
use reqwest;
use base64;
use urlencoding::encode;
use base64::{Engine as _, engine::general_purpose};

#[tokio::main]
async fn create_tag(tag: String, url: String, creds: String) -> Result<i64, Box<dyn Error>> {
    let body = format!("{{\"name\":\"{}\"}}", tag);

    let wordpress: reqwest::Client = reqwest::Client::new();
    let create_tag_response: reqwest::Response = wordpress.post(url)
        .header("Authorization", format!("Basic {}", &creds))
        .body(body)
        .send()
        .await?;

    let create_tag_response: String = create_tag_response.text().await?;
    let create_tag_response: serde_json::Value = serde_json::from_str(&create_tag_response)?;
    match create_tag_response[0]["id"].as_i64() {
        Some(id) => Ok(id),
        None => Err("Something went wrong".into()),
    }
}
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
    print!("{}\n", existing_tags[0]);

    if existing_tags=="[]" { // we must create a new tag
        Ok(create_tag(tag, url, creds)?)
    } else{ // return tag's ID
        let first_tag_id = existing_tags[0]["id"].as_i64().unwrap();
        Ok(first_tag_id)
    }
}

pub fn wordpress(){
    let result = find_tag("gambino".to_string());
    match result {
        Ok(result) => println!("{}", result),
        Err(err) => println!("Error: {}", err),
    }
    // println!("{}", "Called wordpress function".green());
}