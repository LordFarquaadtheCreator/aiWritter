// unofficial rust wrapper for openai api
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4;
use std::env;
use std::fs;
use std::io::{self};
use dotenv::dotenv;
use colored::*;

/// reads & returns content of files
fn read_file(file_path:String) -> io::Result<String> {
    let content: String = fs::read_to_string(file_path)?;
    println!("Got Email Contents");
    Ok(content)
}

/// prompts gpt with email content
pub fn gpt_prompt() -> Result<String, Box<dyn std::error::Error>> {
    // set up variables & client
    dotenv().ok();
    let openai_api_key = env::var("OPENAI_API_KEY").unwrap();
    let client = Client::new(openai_api_key);

    let email_content = read_file("email.txt").map_err(|_| "Failed to get email content")?;
    let gpt_prompt = read_file("gpt_prompt.txt").map_err(|_| "Failed to get gpt prompt content")?;

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
    let result = client.chat_completion(req)?;
    println!("{}", "Prompting Done!".blue());

    // return result
    match result.choices.get(0).and_then(|choice| choice.message.content.clone()) {
        Some(content) => Ok(content),
        None => Err("No content available".into()),
    }
}

