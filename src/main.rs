use futures::future::try_join_all;

mod gpt_prompt;
use gpt_prompt::gpt_prompt;
use gpt_prompt::GPTPrompt;

mod wordpress;
use wordpress::find_tag;
use wordpress::post;
use std::error::Error;

// mod post_to_insta;
// use post_to_insta::post_to_insta;

/// will break if tags have break characters in them!!!
// async fn read_file(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let file_path = format!("./{}", file_name);
//     let file_contents = tokio::fs::read_to_string(file_path).await?;
//     Ok(file_contents)
// }

#[tokio::main]
async fn main (){
    // create post text
    let content_result: Result<GPTPrompt, Box<dyn Error>> = gpt_prompt().await;
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
        
        let tag_id: i64 = match tag_result {
            Ok(id) => id,
            Err(err) => {
                eprintln!("Error: {}", err);
                return Err(err);
            }
        };
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
    if (200..400).contains(&post) {println!("Success!");} 
    else {println!("Failed to create post");}

    // post to instagram
    // let post_result: Result<(), Box<dyn Error>> = post_to_insta("https://cdn.discordapp.com/attachments/1192918386368323757/1193043827250954303/Instagram_Post14.png?ex=65ab47a6&is=6598d2a6&hm=893c6addad3f5bec062e3454f48ba87396e2171d7307c9a34dd0dc2fdd3b6116&".to_string(), "hi hi &^*(".to_string()).await;
    // match post_result {
    //     Ok(_) => println!("Success!"),
    //     Err(err) => eprintln!("Error: {}", err),
    // }

}