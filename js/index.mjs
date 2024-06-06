import fetch from 'node-fetch';
import { promises as fs } from 'fs';
import { createPost } from './gptPrompt.mjs';
import { convertTagsToIDs, wordPressPost } from './wordpress.mjs';
import chalk from 'chalk';

export const handler = async (event, context) => {
    // todo: rewrite in typescript

    try{
        const email = event.body.email;

        // create post
        let post = await createPost(email);
        post = JSON.parse(post);
        post.tags = await convertTagsToIDs(post);

        // publish post
        wordPressPost().then(post => {
            console.log("Success!", post);
        }).catch(error => {
            return (error, 500);
        });

        return (null, 200)
    } catch(e) {
        return (e, 400)
    }
    
    return 201;
};