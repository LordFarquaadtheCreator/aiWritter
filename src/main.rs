mod gpt_prompt;
use gpt_prompt::gpt_prompt;
use gpt_prompt::GPTPrompt;

mod wordpress;
use wordpress::find_tag;
use wordpress::post;
use std::error::Error;

#[tokio::main]
async fn main (){
    // create post text
    let content_result = gpt_prompt();
    let content: GPTPrompt = match content_result {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    // iteratively create & store tags to be used in post
    let tags: Result<Vec<i64>, Box<dyn Error>> = content.tags.iter().map(|tag: &String| {
        let tag_result: Result<i64, Box<dyn Error>> = find_tag(tag.to_string());
        let tag_id: i64 = tag_result?; // Propagate the error if there is one
        Ok(tag_id)
    }).collect();
    let tags: Vec<i64> = match tags {
        Ok(tags) => tags,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    // create post
    let post = post(content, tags).await;
    let post: bool = match post {
        Ok(post) => post,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };
    if post {
        println!("Success!");
    } else {
        println!("Failed to create post");
    }
}