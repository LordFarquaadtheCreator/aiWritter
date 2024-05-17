use cli_clipboard::{ClipboardContext, ClipboardProvider};
use reqwest;

fn copy_to_clipboard(text: &str) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(text.to_owned()).unwrap();
}

fn get_html(topic: &str) -> Result<String, reqwest::Error>{
    let topic: String = topic.trim().replace(" ", "");
    let url: String = format!("https://best-hashtags.com/hashtag/{}/", topic);
    let body: String = reqwest::blocking::get(url)?.text()?;
    
    Ok(body)
}

fn main (){
    copy_to_clipboard("hellooo")
}