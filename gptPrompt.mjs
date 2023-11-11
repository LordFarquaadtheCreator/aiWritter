import {OpenAI} from 'openai';
import chalk from 'chalk';
import dotenv from 'dotenv/config'
import { resolve } from 'path';
import { rejects } from 'assert';
const apiKey = process.env.OPENAI_API_KEY;

function generateChatResponse(apiKey, prompt) {
  return new Promise((resolve, reject) => {
    const openai = new OpenAI({
      apiKey: apiKey,
    });
    openai.chat.completions.create({
      model: 'gpt-4',
      messages: [
        { role: 'system', content: "You will recieve news articles describing a certain topic or event. You will output json in the following order: an appropiate headline with the prefix (WATCH) if the article describes a video or (READ) if it's just a regular article. The the next items in the json will be the body of the article that was passed - but formatted neatly for block-editor in wordpress. It will also contian tags relevant to the post that are seperated by commas. It will also contain a field for a short SEO title for the post, and lastly a field for a 200-char summary of the article."},
        { role: 'user', content: prompt }
      ],
    })
    .then(response => resolve(response))
    .catch(error => reject(error));
  });
}
export async function createPost(input) {
  try {
    console.log(chalk.blue.bgYellow.bold('Prompting GPT'));
    const response = await generateChatResponse(apiKey, String(input));
    console.log(chalk.white.bgGreen("Success!"));
    
    return response;
  } catch (error) {
    console.error('Error:', error);
  }
}