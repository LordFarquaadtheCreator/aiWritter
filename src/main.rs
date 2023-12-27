mod gpt_prompt;
use gpt_prompt::gpt_prompt;
use gpt_prompt::GPTPrompt;
// mod wordpress;
// use wordpress::find_tag;
// use std::error::Error;

fn main (){
    // create post text
    let content_result = gpt_prompt();
    let content: GPTPrompt = match content_result {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    println!("{}", content.title);

    // iteratively create & store tags to be used in post

    // let tag_array: Vec<str> = vec![];

    // let result = find_tag("kendrick lamar".to_string());
}
// async function convertTagsToIDs(post) {
//   if (!post.tags || !Array.isArray(post.tags)) {
//     throw new Error('No tags array found or the tags property is not an array.');
//   }

//   const tagPromises = post.tags.map(tagName => createTag(tagName));
//   const tagIDs = await Promise.all(tagPromises);

//   return tagIDs;
// }
// post.tags = await convertTagsToIDs(post);