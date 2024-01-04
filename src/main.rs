use futures::future::try_join_all;

mod gpt_prompt;
use gpt_prompt::gpt_prompt;
use gpt_prompt::GPTPrompt;

mod wordpress;
use wordpress::find_tag;
use wordpress::post;
use std::error::Error;

/// will break if tags have break characters in them!!!

#[tokio::main]
async fn main (){
    // create post text
    let content_result = gpt_prompt().await;
    let content: GPTPrompt = match content_result {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    // iteratively create & store tags to be used in post
    let tag_futures: Vec<_> = content.tags.iter().map(|tag: &String| async {
        let tag_result = find_tag(tag.to_string()).await;
        let tag_id: i64 = tag_result?; // Propagate the error if there is one
        Ok(tag_id)
    }).collect();

    let tags: Result<Vec<i64>, Box<dyn Error>> = try_join_all(tag_futures).await;
    let tags: Vec<i64> = match tags {
        Ok(tags) => tags,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    // create post
    let post: Result<u16, Box<dyn Error>> = post(content, tags).await;
    let post: u16 = match post {
        Ok(post) => post,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };
    if post > 200 && post < 400 {println!("Success!");} 
    else {println!("Failed to create post");}
}