import fetch from 'node-fetch';
import axios from 'axios';

async function createTag(name) {
  const url = 'https://rightondigital.com/wp-json/wp/v2/tags';
  const credentials = Buffer.from('fahadfaruqi1@gmail.com:' + process.env.PASSWORD).toString('base64');

  // First, try to retrieve the tag by name to see if it already exists
  const existingTagResponse = await fetch(`${url}?search=${encodeURIComponent(name)}`, {
    method: 'GET',
    headers: {
      'Authorization': `Basic ${credentials}`
    }
  });

  if (!existingTagResponse.ok) {
    throw new Error(`HTTP error when searching for tag!: ${existingTagResponse.body}`);
  }

  const existingTags = await existingTagResponse.json();

  // If the tag exists, return the existing ID
  if (existingTags && existingTags.length > 0) {
    return existingTags[0].id;
  }

  // Tag does not exist, create a new one
  const createTagResponse = await axios.post(url, { name: name }, {
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Basic ${credentials}`
    }
  });

  if (createTagResponse.status !== 201 && createTagResponse.status !== 200) {
    throw new Error(`HTTP error when creating tag! status: ${createTagResponse.status}`);
  }

  const newTag = await createTagResponse.data;
  return newTag.id; 
}

// convert unique and duplicate tags to ids
export async function convertTagsToIDs(post) {
  if (!post.tags || !Array.isArray(post.tags)) {
    throw new Error('No tags array found or the tags property is not an array.');
  }

  const tagPromises = post.tags.map(tagName => createTag(tagName));
  const tagIDs = await Promise.all(tagPromises);

  return tagIDs;
}

export const wordPressPost = async () => {
  const url = 'https://rightondigital.com/wp-json/wp/v2/posts';
  const credentials = Buffer.from('fahadfaruqi1@gmail.com:' + process.env.PASSWORD).toString('base64');

  const postData = {
    title: post.title,
    excerpt: post.excerpt,
    content: post.content,
    status: 'draft',
    categories: post.categories, 
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
