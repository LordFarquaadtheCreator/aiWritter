use futures::future::try_join_all;

mod gpt_prompt;
use gpt_prompt::gpt_prompt;
use gpt_prompt::GPTPrompt;

mod wordpress;
use wordpress::find_tag;
use wordpress::post;
// use wordpress::post_image;
use std::error::Error;

// mod post_to_insta;
// use post_to_insta::post_to_insta;

mod get_hashtags;
use get_hashtags::ask_website;

/// reads file & returns contents as string
async fn read_file_to_string(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file_path = format!("./{}", file_name);
    let file_contents = tokio::fs::read_to_string(file_path).await?;
    Ok(file_contents)
}

#[tokio::main]
async fn main (){
    let topic: String = "food".to_string();
    let res:String = ask_website(topic).await;
    println!("{}", res);
    // create post text
    // let content_result: Result<GPTPrompt, Box<dyn Error>> = gpt_prompt().await;
    // let content: GPTPrompt = match content_result {
    //     Ok(content) => content,
    //     Err(err) => {
    //         eprintln!("Error: {}", err);
    //         return;
    //     }
    // };


    // // case: content body has invalid characters - i manually fixed it
    // // let content: String = read_file_to_string("gpt_response_formatted.json").await.unwrap();
    // // let content: GPTPrompt = serde_json::from_str(&content).unwrap();

    // // iteratively create & store tags to be used in post
    // // this can be more efficient - batches 
    // let tag_futures: Vec<_> = content.tags.iter().map(|tag: &String| async {
    //     let tag_result = find_tag(tag.to_string()).await;
        
    //     let tag_id: i64 = match tag_result {
    //         Ok(id) => id,
    //         Err(err) => {
    //             eprintln!("Error: {}", err);
    //             return Err(err);
    //         }
    //     };
    //     Ok(tag_id)
    // }).collect();

    // let tags: Result<Vec<i64>, Box<dyn Error>> = try_join_all(tag_futures).await;
    // let tags: Vec<i64> = match tags {
    //     Ok(tags) => tags,
    //     Err(err) => {
    //         eprintln!("Error: {}", err);
    //         return;
    //     }
    // };

    // // // img_id
    // // let img_id = post_image("https://mail.google.com/mail/u/0?ui=2&ik=67634cf87e&attid=0.2&permmsgid=msg-f:1788368980422726673&th=18d1901ccd6c9411&view=fimg&fur=ip&sz=s0-l75-ft&attbid=ANGjdJ9EerK8r8XwB30B_wqzItfVozlutVZXOqkZ-ymMp50lz29_5xkrrPYIg3Maud6phd_Wi6QbJ4rJ5Mhu38dezoSgV8xEaGfi8MSpin26ccSu1YRksLMIXz0pW24&disp=emb&realattid=ii_18d18ff796490cac092".to_string());
    // // img_id.await;
    // // let img_id: String = match img_id {
    // //     Ok(img_id) => img_id,
    // //     Err(err) => {
    // //         eprintln!("Error: {}", err);
    // //         return;
    // //     }
    // // };
    // // println!("img_id: {}", img_id);

    // // create post
    // let post: Result<u16, Box<dyn Error>> = post(content, tags).await;
    // let post: u16 = match post {
    //     Ok(post) => post,
    //     Err(err) => {
    //         eprintln!("Error: {}", err);
    //         return;
    //     }
    // };
    // if (200..400).contains(&post) {println!("Success!");} 
    // else {println!("Failed to create post");}

    // post to instagram
    // let post_result: Result<(), Box<dyn Error>> = post_to_insta("https://cdn.discordapp.com/attachments/1192918386368323757/1193043827250954303/Instagram_Post14.png?ex=65ab47a6&is=6598d2a6&hm=893c6addad3f5bec062e3454f48ba87396e2171d7307c9a34dd0dc2fdd3b6116&".to_string(), "hi hi &^*(".to_string()).await;
    // match post_result {
    //     Ok(_) => println!("Success!"),
    //     Err(err) => eprintln!("Error: {}", err),
    // }

}