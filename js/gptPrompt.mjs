import {OpenAI} from 'openai';
import gptPromptStr from './gptPromptStr.mjs';

// TODO: add llm functions?
async function generateChatResponse(apiKey, prompt) {
  const openai = new OpenAI({apiKey: apiKey,});
  const systemPrompt = gptPromptStr;
  console.log('Prompting GPT');
  
  const res = openai.chat.completions.create({
    model: 'gpt-4o',
    messages: [
      { role: 'system', content: systemPrompt},
      { role: 'user', content: prompt }
    ],
    response_format:{ "type": "json_object" }
  })
  return res;
}

export async function createPost(input) {
  try {
    console.log("Creating Post!");

    const apiKey = process.env.OPENAI_API_KEY;
    const response = await generateChatResponse(apiKey, String(input));

    console.log("Success Generating JSON!");

    return response.choices[0].message.content;
  } catch (error) {
    throw Error(error);
  }
}