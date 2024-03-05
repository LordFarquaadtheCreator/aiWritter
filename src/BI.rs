use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct GPTPrompt {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub categories: Vec<i64>,
    pub excerpt: String,
}
/// removes invalid characters from object subitems (String only)
pub fn prune_characters(object:Vec<String>) -> Vec<String> {
    let mut new_object: Vec<String> = Vec::new();

    for item in object {
        let new_item: String = item.replace("&", "").replace("+", "").replace("?", "").replace("/", "").replace("\\", "").replace("^", "").replace("$", "").replace("[", "").replace("]", "").replace("{", "").replace("}", "").replace("(", "").replace(")", "").replace("|", "");
        new_object.push(new_item);
    }
    new_object
}
/// saves content to file
/// obj_type MUST be valid type (no error checking)
pub fn save(content: String, file_name: String, obj_type: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::create(format!("./{file_name}.{obj_type}"))?;
    let _ = file.write_all(content.as_bytes())?;
    Ok(format!("./{file_name}.{obj_type}").to_string())
}
/// reads file & returns contents as string
pub fn read(file_name: String) -> Result<String, Box<dyn std::error::Error>> {
    let file_path: String = format!("./{}", file_name);
    let file_contents: String = std::fs::read_to_string(file_path)?;
    Ok(file_contents)
}