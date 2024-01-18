use dotenv::dotenv;
use std::{env, error::Error};
// use colored::*;
use reqwest::{self, header::{ACCEPT, CONTENT_TYPE, AUTHORIZATION, CONTENT_DISPOSITION}};
use base64;
use urlencoding::encode;
use base64::{Engine as _, engine::general_purpose};
use serde::{Serialize, Deserialize};
use reqwest::blocking::get;

use crate::GPTPrompt;

// allows us to convert to string
#[derive(Serialize, Deserialize, Debug)]
struct PostData{
    title: String,
    excerpt: String,
    content: String,
    status: String,
    categories: Vec<i64>,
    tags: Vec<i64>,
}

/// returns tag's ID if it exists
pub async fn find_tag(tag: String) -> Result<i64, Box<dyn Error>> {
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
            Err(err) => Err(format!("Error creating tag for {tag}.\n{err}").into()),
        }
    } else { // array not empty so return tag's ID
        match existing_tags[0]["id"].as_i64() {
            Some(id) => Ok(id),
            None => Err("Invalid ID".into()),
        }
    }
}

/// creates a new tag in wordpress
/// returns tag's ID
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

/// posts image to wordpress,
/// returns image id
pub async fn post_image(image_path: String) -> Result<String, Box<dyn Error>>{
    let img = get(image_path).expect("Failed to get image");
    let img_bytes = img.bytes().expect("Failed to convert response into bytes");
    
    let url = "https://rightondigital.com/wp-json/wp/v2/posts";
    let password: String = env::var("PASSWORD").unwrap().to_string();
    let creds = general_purpose::STANDARD.encode(format!("fahadfaruqi1@gmail.com:{}", password));

    let wordpress: reqwest::Client = reqwest::Client::new();
    let post_image: reqwest::Response = wordpress.post(url)
    .header(AUTHORIZATION, format!("Basic {}", &creds))
    .header(CONTENT_TYPE, "image/jpeg")
    .header(ACCEPT, "application/json")
    .header(CONTENT_DISPOSITION, "attachment; filename=img.jpg")
    .body(img_bytes)
    .send()
    .await?;

    let wordpress_response: String = post_image.text().await?;
    println!("{}", wordpress_response);

    Ok(wordpress_response)
}

/// returns status code of post request
pub async fn post(content: GPTPrompt, tags: Vec<i64>) -> Result<u16, Box<dyn Error>> {
    dotenv().expect("Failed to read .env file");
    let url = "https://rightondigital.com/wp-json/wp/v2/posts";
    let password: String = env::var("PASSWORD").unwrap().to_string();
    let creds = general_purpose::STANDARD.encode(format!("fahadfaruqi1@gmail.com:{}", password));
    // println!("{:#?}", content);

    // convert post data into an object just to enforce structure
    let post_data = PostData{
        title: content.title,
        excerpt: content.excerpt,
        content: content.content,
        status: "draft".to_string(),
        categories: content.categories,
        tags: tags
    };
    // println!("{:#?}", post_data);

    // using the .json 
    let wordpress: reqwest::Client = reqwest::Client::new();
    let submit_to_wordpress: reqwest::Response = wordpress.post(url)
        .header("Authorization", format!("Basic {}", &creds))
        .header("Content-Type", "application/json")
        // .body(post_data_string)
        .json(&post_data)
        .send()
        .await?;

    // let wordpress_response: String = submit_to_wordpress.text().await?;
    // println!("{}", wordpress_response);
    Ok(submit_to_wordpress.status().as_u16())
}