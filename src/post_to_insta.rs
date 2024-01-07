use dotenv::dotenv;
use std::env;
use urlencoding::encode;
use colored::*;

async fn publish_container(post_id:String, graph_access_token: &str, graph_user_id: &str) -> Result<(), Box<dyn std::error::Error>>{
    let url = format!("https://graph.facebook.com/v18.0/{}/media_publish?creation_id={}&access_token={}", graph_user_id, post_id, graph_access_token);
    
    let publish_container: reqwest::Client = reqwest::Client::new();
    let publish_container: reqwest::Response = publish_container.post(url)
        .send()
        .await
        .unwrap();
    println!("Publish Container Status: {}", publish_container.status());
    Ok(())
}

async fn create_container(photo_url: String, caption: String, graph_access_token: &str, graph_user_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let encoded_caption: String = encode(&caption).to_string();
    // assumption: url and token & id are valid encoded
    let url: String = format!("https://graph.facebook.com/v18.0/{}/media?image_url={}&caption={}&access_token={}", graph_user_id, photo_url, encoded_caption, graph_access_token);

    // send request to create container
    let container: reqwest::Client = reqwest::Client::new();
    let upload_container: reqwest::Response = container.post(url)
        .send()
        .await?;

    // get upload id from response
    let upload_id = match upload_container.text().await{
        Ok(upload_id) => {
            if upload_id.contains("error"){
                return Err(upload_id.into());
            } else{
                let upload_json: serde_json::Value = serde_json::from_str(&upload_id)?;
                let upload_id = upload_json.get("id").unwrap().to_string().trim_matches('"').to_string();
                upload_id
            }
        },
        Err(err) => return Err(err.into()),
    };

    Ok(upload_id)
}

pub async fn post_to_insta(photo_url: String, caption: String) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("Failed to read .env file");
    let graph_access_token: String = env::var("GRAPH_ACCESS_TOKEN").unwrap().to_string(); 
    let graph_user_id: String = env::var("GRAPH_USER_ID").unwrap().to_string();
    println!("{}", "Warning: Graph API Website must be open in browser to post to Instagram!".bold());

    let result_container: Result<String, Box<dyn std::error::Error>> = create_container(photo_url, caption, &graph_access_token, &graph_user_id).await;
    let post_id = match result_container{
        Ok(post_id) => post_id,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
            // return Err(err);
        }
    };
    println!("Post ID: {}", post_id);

    let result_publish: Result<(), Box<dyn std::error::Error>> = publish_container(post_id, &graph_access_token, &graph_user_id).await;
    match result_publish{
        Ok(_) => println!("Published!"),
        Err(err) => {
            eprintln!("Error: {}", err);
            return Err(err);
        }
    }
    Ok(())
}