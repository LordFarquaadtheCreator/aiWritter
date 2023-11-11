import fetch from 'node-fetch';
import dotenv from 'dotenv/config';
import { promises as fs } from 'fs';
import { createPost } from './gptPrompt.mjs';
import { createTag } from './wordpress.mjs';

// get article from email.txt
console.log("Reading Article!");
async function readEmail() {
    try {
        const data = await fs.readFile('./email.txt', 'utf8');
        return data; 
    } catch (error) {
        console.error('Error reading the file:', error);
        throw error; 
    }
}
const email = (await readEmail());
console.log("Article Read!");

// process article
console.log("Creating Post!");
const post = JSON.parse(await createPost(email));

// convert unique and duplicate tags to ids
async function convertTagsToIDs(post) {
  if (!post.tags || !Array.isArray(post.tags)) {
    throw new Error('No tags array found or the tags property is not an array.');
  }

  const tagPromises = post.tags.map(tagName => createTag(tagName));
  const tagIDs = await Promise.all(tagPromises);

  return tagIDs;
}
post.tags = await convertTagsToIDs(post);



const wordPressPost = async () => {
    const url = 'https://rightondigital.com/wp-json/wp/v2/posts';
    const credentials = Buffer.from('fahadfaruqi1@gmail.com:' + process.env.PASSWORD).toString('base64');

    const postData = {
        title: post.title,
        excerpt: post.excerpt,
        content: post.content,
        status: 'draft',
        categories: post.categories, // This should be an array of category IDs
        tags: post.tags
    };

    const response = await fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Basic ${credentials}`
        },
        body: JSON.stringify(postData)
    });

    if (!response.ok) {
        const message = `An error has occurred: ${response.status}`;
        throw new Error(message);
    }

    const postResponse = await response.json();
    return postResponse;
};

wordPressPost().then(post => {
    console.log("\x33[92mSuccess!\x33[0m!");
}).catch(error => {
    console.error('Error creating post:', error);
});
