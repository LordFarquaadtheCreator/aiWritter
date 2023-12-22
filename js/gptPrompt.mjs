import {OpenAI} from 'openai';
import chalk from 'chalk';
import dotenv from 'dotenv/config'
import { resolve } from 'path';
import { rejects } from 'assert';
import fs from 'fs';
const apiKey = process.env.OPENAI_API_KEY;

function generateChatResponse(apiKey, prompt) {
  return new Promise((resolve, reject) => {
    const openai = new OpenAI({
      apiKey: apiKey,
    });
    openai.chat.completions.create({
      model: 'gpt-4',
      messages: [
        { role: 'system', content: "You will recieve news articles describing a certain topic or event. Ignore all text that discusses the text being an email. In your response you must strictly adhere to the following rules: the text must be formatted neatly as a JSON file as well as avoiding bad characters that would mess up input into a json object and sending via api. You must not include double quotation marks or extra commas that will break the JSON body. Ensure there are no bad control characters in the body - it must be a proper json data object. You will output json in the following order: an appropiate headline (called 'title') with the prefix (WATCH) if the article describes a video or (READ) if it's just a regular article, (PICS) if its a gallery of images, and (BUY) if it's an advertisement. The next item in the json will be the body (called 'content') of the article that was passed. Feel free to generate some extra text to flesh out the post - make it as interesting and full of information (information found on the email) as possible. It will also contain an array of tags called 'tags' relevant to the post. An array of categories (called 'categories') that the article fits into the following categories gets turned into the respective number it points to (EX: if the text fits BOOKS, convert it to 67): [GAMES-351,MUSIC & VIDEOS-3, BOOKS-67, BREAKING NEWS-12, CULTURE & EVENTS-8, FILM & TV-9, HEALTH & FITNESS-80, PHOTO GALLERY-2, STYLE & BEAUTY-1819, Gift Guide-1819, Review-4894]. A post can have more than one category. Lastly a field for a 200-char summary (called 'excerpt')of the article."},
        { role: 'user', content: prompt }
      ],
      // response_format:{ "type": "json_object" }
    })
    .then(response => resolve(response))
    .catch(error => reject(error));
  });
}
// function fixJson(json){

//   const lines = json.split('\n');
//   let fixedLines = [];

//   for (let i = 0; i < lines.length; i++) {
//       let line = lines[i].trim();
      
//       // Skip empty lines
//       if (line === '') continue;

//       // Fix extra quotation marks
//       line = line.replace(/"{2,}/g, '"');

//       // Ensure commas are correctly placed
//       if (i < lines.length - 1 && !line.endsWith(',') && !line.endsWith('{') && !line.endsWith('[')) {
//           const nextLine = lines[i + 1].trim();
//           if (nextLine && nextLine !== '}' && nextLine !== ']') {
//               line += ',';
//           }
//       }
//       fixedLines.push(line);
//   }

//   return(fixedLines.join('\n'));
// }
// async function readEmail() {
//     try {
//         await fs.readFile('./gpt.txt', 'utf8', function(err, data) {
//           if (err) {
//             console.error(err);
//             return;
//           }
//           return data; 
//         });
//     } catch (error) {
//         console.error('Error reading the file:', error);
//         throw error; 
//     }
// }
// const text = (await readEmail());
// console.log(text);

export async function createPost(input) {
  try {
    console.log(chalk.blue.bgYellow.bold('Prompting GPT'));
    const response = await generateChatResponse(apiKey, String(input));
    console.log(chalk.white.bgGreen("Success Generating JSON!"));

    return response.choices[0].message.content;
  } catch (error) {
    console.error('Error:', error);
  }
}
// createPost("hello");