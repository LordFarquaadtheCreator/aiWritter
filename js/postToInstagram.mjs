import axios from 'axios';
import dotenv from 'dotenv/config';
import fs from 'fs';
import path from 'path';

function readFile(fileName){
    const data = fs.readFileSync(`./${fileName}`, 'utf-8');
    return data;
}

async function createContainer(photoUrl, hashtags){
    const caption = encodeURIComponent(`Yeah we did that! Check this out and more with the link in bio!\n.\n.\n.\n${hashtags}`);
    const userId = process.env.GRAPH_USER_ID;
    const accessToken = process.env.GRAPH_ACCESS_TOKEN;
    let postId = '';

    const url = `https://graph.facebook.com/v18.0/${userId}/media?image_url=${photoUrl}&caption=${caption}&access_token=${accessToken}`;
    try {
        const response = await axios.post(url); // no error handling
        // console.log(response);
        postId = response.data.id;
        // console.log('Post ID:', postId);
    } catch (error) {
        console.error('Error:', error);
    }
    return postId;
};
async function publishContainer(postId){
    const userId = process.env.GRAPH_USER_ID;
    const accessToken = process.env.GRAPH_ACCESS_TOKEN;
    const url = `https://graph.facebook.com/v18.0/${userId}/media_publish?creation_id=${postId}&access_token=${accessToken}`;
    let success = false;

    try{
        await axios.post(url)
        .then((response) => {
            success = true
        })
        .catch((error) =>{
            console.log(error);
        });
    } catch (error) {
        console.error('Error:', error.message);
    }

    return success;
}
async function postToInstagram(photoUrl, hashtags){
    let fail = false;
    const postId = await createContainer(photoUrl, hashtags);
    
    postId != '' ? console.log("Post created: ", postId) : fail=true;
    if(fail) process.exit()

    const success = await publishContainer(postId);
    !success ? console.log("Post failed to publish") : console.log("Post published successfully");

    return success;
}
const args = process.argv.slice(2);
const photoUrl = args[0];
if(!photoUrl) {
    console.log("No photo url provided - first arg pls");
    process.exit();
}
await postToInstagram(photoUrl, readFile('hashtags.txt'))