// mod gpt_prompt;
// use gpt_prompt::gpt_prompt;
mod wordpress;
use wordpress::wordpress;
use std::error::Error;

fn main (){
    wordpress();
    // println!("{}", result.unwrap());
}