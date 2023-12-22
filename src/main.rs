use colored::Colorize;
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4;
use std::env;
use std::process;
use std::fs;
use std::io::{self, Read};
use dotenv::dotenv;
use colored::*;

fn get_email() -> io::Result<String> {
    let file_path = "email.txt";
    let content: String = fs::read_to_string(file_path)?;
    println!("Got Email Contents");
    Ok(content)
}

fn main() -> Result<(), Box<dyn std::error::Error>> { // this means we can debug in main
    // set up variables & client
        dotenv().ok();
    let openai_api_key = env::var("OPENAI_API_KEY").unwrap();
    let client = Client::new(openai_api_key);
    let email_content = get_email().expect("Failed to get email content");

    // just incase the file is empty
    if email_content == "Failed to get email content"{process::exit(1);}

    // set up request
    let req: ChatCompletionRequest = ChatCompletionRequest::new(
        GPT4.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            name: None,
            function_call: None,

            // The AI's role defined
            role: chat_completion::MessageRole::system,
            content: String::from("You will recieve news articles describing a certain topic or event. In your response you will ignore all text that discusses the text being an email. You will output json in the following order: an appropiate headline (called 'title') with the prefix (WATCH) if the article describes a video or (READ) if it's just a regular article, (PICS) if its a gallery of images, and (BUY) if it's an advertisement. The next item in the json will be the body (called 'content') of the article that was passed - but formatted neatly in HTML for in wordpress as well as avoiding bad characters that would mess up input into a json object and sending via api. Ensure there are no bad control characters in the body - it must be a proper json data object. Feel free to generate some extra text to flesh out the post - make it as interesting and full of information (information found on the email) as possible. It will also contain an array of tags called 'tags' relevant to the post. An array of categories (called 'categories') that the article fits into the following categories gets turned into the respective number it points to (EX: if the text fits BOOKS, convert it to 67): [GAMES-351,MUSIC & VIDEOS-3, BOOKS-67, BREAKING NEWS-12, CULTURE & EVENTS-8, FILM & TV-9, HEALTH & FITNESS-80, PHOTO GALLERY-2, STYLE & BEAUTY-1819, Gift Guide-1819, Review-4894]. A post can have more than one category. Lastly a field for a 200-char summary (called 'excerpt')of the article. In your response only provide the json code, nothing else."),
        }, chat_completion::ChatCompletionMessage{
            name: None,
            function_call: None,

            // The prompt to generate a completion for
            role: chat_completion::MessageRole::user,
            content: email_content,
            }
        ]
    );

    // send & print request
    let result = client.chat_completion(req)?;
    let message: Option<String> = result.choices[0].message.content.clone();
    match message {
        Some(content) => println!("{}", content.green()),
        None => println!("No message found"),
    }

    //all good
    Ok(())
}

