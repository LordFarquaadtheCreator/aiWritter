import {OpenAI} from 'openai';
import chalk from 'chalk';
import dotenv from 'dotenv/config'
import { resolve } from 'path';
import { rejects } from 'assert';
import fs from 'fs';

function getPrompt() {
  let str = fs.readFileSync('gpt_prompt.txt', 'utf8');
  return str;
}

function generateChatResponse(apiKey, prompt) {
  return new Promise((resolve, reject) => {
    const openai = new OpenAI({apiKey: apiKey,});
    const systemPrompt = getPrompt();

    openai.chat.completions.create({
      model: 'gpt-4o',
      messages: [
        { role: 'system', content: systemPrompt},
        { role: 'user', content: prompt }
      ],
      response_format:{ "type": "json_object" }
    })
    .then(response => resolve(response))
    .catch(error => reject(error));
  });
}

export async function createPost(input) {
  try {
    const apiKey = process.env.OPENAI_API_KEY;

    console.log(chalk.blue.bgYellow.bold('Prompting GPT'));

    const response = await generateChatResponse(apiKey, String(input));

    console.log(chalk.white.bgGreen("Success Generating JSON!"));

    return response.choices[0].message.content;
  } catch (error) {
    throw Error(error.toString());
  }
}