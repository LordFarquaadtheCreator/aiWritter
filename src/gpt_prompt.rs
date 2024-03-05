use serde_json::json;
use std::env;
use std::io::{self};
use dotenv::dotenv;
use colored::*;
use reqwest::header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION};

use crate::BI;
use BI::GPTPrompt;
use BI::prune_characters;
use BI::{save, read};

// prompts gpt with email content
async fn generate_response() -> Result<String, Box<dyn std::error::Error>> {
    // set up variables & client
    dotenv().ok();
    let openai_api_key = env::var("OPENAI_API_KEY").unwrap();
    let url = "https://api.openai.com/v1/chat/completions".to_string();
    let email_content: String = read("email.txt".to_string()).map_err(|_| "Failed to get email content")?;
    let gpt_prompt = read("gpt_prompt.txt".to_string()).map_err(|_| "Failed to get gpt_prompt.txt content")?;

    if email_content.trim().is_empty() || gpt_prompt.trim().is_empty(){
        return Err("Email or gpt prompt content is empty!".into());
    }

    // send request to openai
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Bearer {}", openai_api_key).parse().unwrap());

    let request_data = json!({
        "model": "gpt-4-1106-preview",
        "messages": [
            {
                "role": "system",
                "content": gpt_prompt
            },
            {
                "role": "user",
                "content": email_content
            }
        ],
        "response_format": { "type": "json_object" },
        "temperature": 1.1
    });

    println!("{}", "Prompting GPT...".green());
    let response = client.post(url)
        .headers(headers)
        .body(request_data.to_string())
        .send()
        .await?
        .text()
        .await?;

    let jresponse: serde_json::Value = serde_json::from_str(&response)?;
    let completion = jresponse["choices"][0]["message"]["content"].as_str().unwrap_or("");
    let _ = save(completion.to_string(), "gpt_response".to_string(), "json".to_string());
    println!("{}", "Prompting Done!".blue());

    // convert response to string & return
    Ok(completion.to_string())
}

// convert ChatCompletionResponse into GPTPrompt struct
fn parse_reponse (gpt_reponse: String) -> Result<GPTPrompt, Box<dyn std::error::Error>> {
    // convert to json
    println!("Converting to json");
    let gpt_body: serde_json::Value = serde_json::from_str(&gpt_reponse)?;
    // println!("{:#?}", gpt_body);
    
    // ensuring that vectors are vectors of strings
    let tags: Result<Vec<String>, _> = gpt_body["tags"].as_array()
        .ok_or("Expected an array")?
        .iter()
        .map(|v| v.as_str().ok_or("Expected a string").map(|s| s.to_owned()))
        .collect();

    let categories: Result<Vec<i64>, _> = gpt_body["categories"].as_array()
        .ok_or("Expected an array")?
        .iter()
        .map(|v| v.as_i64().ok_or("Expected a string").map(|s| s.to_owned()))
        .collect();

    let response: GPTPrompt = GPTPrompt{
        title: gpt_body["title"].as_str().unwrap().to_string(),
        content: gpt_body["content"].as_str().unwrap().to_string(),
        tags: tags.unwrap(),
        categories: categories.unwrap(),
        excerpt: gpt_body["excerpt"].as_str().unwrap().to_string(),
    };
    println!("Saved response to GPTPrompt body");
    let _ = save(serde_json::to_string(&response).unwrap(), "formatted_gpt_response".to_string(), "json".to_string());

    Ok(response)
}

/// prompts gpt with email content
pub async fn gpt_prompt() -> Result<GPTPrompt, Box<dyn std::error::Error>> {
    // prompts gpt & returns response in string
    let gpt_response: Result<String, Box<dyn std::error::Error>> = generate_response().await;
    let gpt_body: String = match gpt_response {
        Ok(body) => body,
        Err(err) => return Err(format!("Error prompting GPT: {}", err).into()),    
    };    
    // let gpt_body: String = read("gpt_response_.json".to_string()).map_err(|_| "Failed to get gpt response content")?;

    // parse response into GPTPrompt struct
    let gpt_reponse: Result<GPTPrompt, Box<dyn std::error::Error>> = parse_reponse(gpt_body);    
    let response: GPTPrompt = match gpt_reponse {
        Ok(response) => response,
        Err(err) => return Err(format!("Error converting GPT response into GPTbody\n {}", err).into()),
    };
    
    // return result
    Ok(response)
}

