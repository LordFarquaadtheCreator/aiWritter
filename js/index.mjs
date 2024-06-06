import { createPost } from './gptPrompt.mjs';
import { convertTagsToIDs, wordPressPost } from './wordpress.mjs';

export const handler = async (event, context) => {
    // todo: rewrite in typescript

    try {
        const email = event.body.email;

        console.log('Creating post...');
        let post = await createPost(email);
        console.log('Post created:', post);
        
        post = JSON.parse(post);
        post.tags = await convertTagsToIDs(post);

        console.log('Publishing post...');
        await wordPressPost(post).then(result => {
            console.log("Success!", result);
        }).catch(error => {
            console.error("Error publishing post:", error);
            return { statusCode: 500, body: error.toString() };
        });

        return { statusCode: 200, body: 'Success' };
    } catch (e) {
        console.error("Error in handler:", e);
        return { statusCode: 400, body: e.toString() };
    }
};
