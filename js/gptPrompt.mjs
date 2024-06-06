import {OpenAI} from 'openai';
import gptPromptStr from './gptPromptStr.mjs';

// TODO: add llm functions?
async function generateChatResponse(apiKey, prompt) {
  const openai = new OpenAI({ apiKey: apiKey });
  const systemPrompt = gptPromptStr;
  console.log('Prompting GPT');

  try {
    const res = await openai.chat.completions.create({
      model: 'gpt-4o',
      messages: [
        { role: 'system', content: systemPrompt },
        { role: 'user', content: prompt }
      ],
      response_format: { "type": "json_object" }
    });
    return res;
  } catch (error) {
    console.error('Error generating chat response:', error);
    throw error;
  }
}

export async function createPost(input) {
  try {
    console.log("Creating Post!");

    const apiKey = process.env.OPENAI_API_KEY;
    const response = await generateChatResponse(apiKey, String(input));

    console.log("Success Generating JSON!", response.choices[0].message.content);

    return response.choices[0].message.content;
  } catch (error) {
    console.error('Error creating post:', error);
    throw Error(error);
  }
}