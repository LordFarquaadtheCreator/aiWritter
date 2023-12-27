// unofficial rust wrapper for openai api
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4;
use std::env;
use std::fs;
use std::io::{self};
use dotenv::dotenv;
use colored::*;
use std::fs::File;
use std::io::prelude::*;

pub struct GPTPrompt {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub categories: Vec<i64>,
    pub excerpt: String,
}

// reads & returns content of files
fn read_file(file_path:String) -> io::Result<String> {
    let content: String = fs::read_to_string(file_path)?;
    Ok(content)
}

/// prompts gpt with email content
pub fn gpt_prompt() -> Result<GPTPrompt, Box<dyn std::error::Error>> {
    // set up variables & client
    dotenv().ok();
    let openai_api_key = env::var("OPENAI_API_KEY").unwrap();
    let client = Client::new(openai_api_key);
    
    let email_content = read_file("./email.txt".to_string()).map_err(|_| "Failed to get email content")?;
    let gpt_prompt = read_file("./gpt_prompt.txt".to_string()).map_err(|_| "Failed to get gpt prompt content")?;

    if email_content.trim().is_empty() {
        return Err("Email content is empty!".into());
    }

    println!("{}", "Prompting GPT...".green());
    // set up request
    let req: ChatCompletionRequest = ChatCompletionRequest::new(
        GPT4.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            name: None,
            function_call: None,

            // The AI's role defined
            role: chat_completion::MessageRole::system,
            content: String::from(gpt_prompt),
        }, chat_completion::ChatCompletionMessage{
            name: None,
            function_call: None,

            // The prompt to generate a completion for
            role: chat_completion::MessageRole::user,
            content: email_content,
            }
        ]
    );

    // send request
    let result: chat_completion::ChatCompletionResponse = client.chat_completion(req)?;
    println!("{}", "Prompting Done!".blue());

    // parse response
    let gpt_result: String = result.choices.get(0).and_then(|choice| choice.message.content.clone()).unwrap_or("".to_string());
    let gpt_body: serde_json::Value = serde_json::from_str(&gpt_result)?;

    // save response to text file for debugging
    let gpt_body_string = serde_json::to_string(&gpt_body).unwrap();
    let mut file = File::create("./gpt_response.json")?;
    file.write_all(&gpt_body_string.as_bytes())?;
    println!("{}", "Saved reponse to JSON".bold());

    // parse response into GPTPrompt struct
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
        title: gpt_body["title"].to_string(),
        content: gpt_body["content"].to_string(),
        tags: tags?,
        categories: categories?,
        excerpt: gpt_body["excerpt"].to_string(),
    };
    // return result
    Ok(response)
}

