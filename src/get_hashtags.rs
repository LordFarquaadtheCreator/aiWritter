use scraper::{Selector, Html};
use regex::Regex;

fn parse_html(html: &str) -> Vec<String> {
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("div.tag-box").unwrap();
    let mut hashtags = String::new();

    for element in fragment.select(&selector) {
        hashtags.push_str(&element.text().collect::<String>());
    }

    let re = Regex::new(r"#\w+").unwrap();
    let mut matches: Vec<String> = re.find_iter(&hashtags)
        .map(|mat| mat.as_str().to_string())
        .collect();

    matches.truncate(30);
    matches
}

pub fn ask_website (topic: String){
    let client = reqwest::Client::new();
    let url = "https://best-hashtags.com/hashtags/".to_string() + &topic;

    let response = client.post(url).send().await?.text().await?;
    println!("{}", response);
}