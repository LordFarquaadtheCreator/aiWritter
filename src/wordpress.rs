use dotenv::dotenv;
use std::{env, error::Error};
// use colored::*;
use reqwest;
use base64;
use urlencoding::encode;
use base64::{Engine as _, engine::general_purpose};
use serde::{Serialize, Deserialize};

use crate::GPTPrompt;

#[derive(Serialize, Deserialize)]

struct PostData{
    title: String,
    excerpt: String,
    content: String,
    status: String,
    categories: Vec<i64>,
    tags: Vec<i64>,
}

#[tokio::main]
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

pub async fn post(content: GPTPrompt, tags: Vec<i64>) -> Result<bool, Box<dyn Error>> {
    dotenv().expect("Failed to read .env file");
    let url = "https://rightondigital.com/wp-json/wp/v2/posts";
    let password: String = env::var("PASSWORD").unwrap().to_string();
    let creds = general_purpose::STANDARD.encode(format!("fahadfaruqi1@gmail.com:{}", password));

    // convert post data into an object just to enforce structure
    let post_data = PostData{
        title: content.title,
        excerpt: content.excerpt,
        content: content.content,
        status: "draft".to_string(),
        categories: content.categories,
        tags: tags
    };
    // convert post data into a string
    let post_data_string = serde_json::to_string(&post_data)?;


    let wordpress: reqwest::Client = reqwest::Client::new();
    let submit_to_wordpress: reqwest::Response = wordpress.get(url)
        .header("Authorization", format!("Basic {}", &creds))
        .body(post_data_string)
        .send()
        .await?;
    let wordpress_response: String = submit_to_wordpress.text().await?;
    println!("{}", wordpress_response);
    Ok(true)
}
//     const response = await fetch(url, {
//         method: 'POST',
//         headers: {
//             'Content-Type': 'application/json',
//             'Authorization': `Basic ${credentials}`
//         },
//         body: JSON.stringify(postData)
//     });

//     if (!response.ok) {
//         const message = `An error has occurred: ${response.status}`;
//         throw new Error(message);
//     }

//     const postResponse = await response.json();
//     return postResponse;
// };

