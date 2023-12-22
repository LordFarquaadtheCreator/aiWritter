use dotenv::dotenv;
use std::{env, error::Error};
use colored::*;
use reqwest;
use base64;

#[tokio::main]
async fn create_tag(tag: String) -> Result<String, Box<dyn Error>> {
    let password = env::var("PASSWORD")?; // Handle potential error
    let url = "https://rightondigital.com/wp-json/wp/v2/tags";
    let creds = format!("fahadfaruqi1@gmail.com:{}", password);
    let encoded_creds = base64::encode(creds); // Basic Auth requires Base64 encoding

    // query wordpress for all avaliable tags
    let wordpress: reqwest::Client = reqwest::Client::new();
    let response = wordpress.post(url)
        .query(&[("search", &tag)])
        .header("Authorization", format!("Basic {}", encoded_creds))
        .send()
        .await?;


    let existingTags: String = response.text();
    println!("{}", existingTags);

    // Do something with `body` or return a meaningful result
    Ok(true);
}
//   const existingTags = await existingTagResponse.json();

//   // If the tag exists, return the existing ID
//   if (existingTags && existingTags.length > 0) {
//     return existingTags[0].id;
//   }

//   // Tag does not exist, create a new one
//   const createTagResponse = await fetch(url, {
//     method: 'POST',
//     headers: {
//       'Content-Type': 'application/json',
//       'Authorization': `Basic ${credentials}`
//     },
//     body: JSON.stringify({ name: name }) // "name" is the only required field to create a tag
//   });

//   if (!createTagResponse.ok) {
//     throw new Error(`HTTP error when creating tag! status: ${createTagResponse.status}`);
//   }

//   const newTag = await createTagResponse.json();
//   return newTag.id; // Returns the new tag ID

pub fn main(){
    dotenv().ok();
    create_tag("drake".to_string());
    println!("{}", "Wordpress".green());
}