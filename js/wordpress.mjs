import fetch from 'node-fetch';
import dotenv from 'dotenv/config';
import { promises as fs } from 'fs';

export async function createTag(name) {
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
    throw new Error(`HTTP error when searching for tag! status: ${existingTagResponse.status}`);
  }

  const existingTags = await existingTagResponse.json();

  // If the tag exists, return the existing ID
  if (existingTags && existingTags.length > 0) {
    return existingTags[0].id;
  }

  // Tag does not exist, create a new one
  const createTagResponse = await fetch(url, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Basic ${credentials}`
    },
    body: JSON.stringify({ name: name }) // "name" is the only required field to create a tag
  });

  if (!createTagResponse.ok) {
    throw new Error(`HTTP error when creating tag! status: ${createTagResponse.status}`);
  }

  const newTag = await createTagResponse.json();
  return newTag.id; // Returns the new tag ID
}