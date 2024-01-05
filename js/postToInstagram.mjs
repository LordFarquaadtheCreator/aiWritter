import axios from 'axios';
import dotenv from 'dotenv/config';

async function createContainer(photoUrl, hashtags){
    const caption = encodeURIComponent(`Yeah we did that! Check this out and more with the link in bio!\n.\n.\n.\n${hashtags}`);
    const userId = process.env.GRAPH_USER_ID;
    const accessToken = process.env.GRAPH_ACCESS_TOKEN;

    let postId = '';

    const url = `https://graph.facebook.com/v18.0/${userId}/media?image_url=${photoUrl}&caption=${caption}&access_token=${accessToken}`;
    axios.post(url)
    .then((response) => {
        response.status == 200 ? postId = response.data.id : console.log("Error: ", response);
    })
    .catch((error) =>{
        console.log(error);
    });
    
    return postId;
};
async function publishContainer(postId){
    const userId = process.env.GRAPH_USER_ID;
    const accessToken = process.env.GRAPH_ACCESS_TOKEN;
    const url = `https://graph.facebook.com/v18.0/${userId}/media_publish
  ?creation_id=${postId}&access_token=${accessToken}`;
    const success = false;

    axios.post(url)
    .then((response) => {
        response.status == 200 ? success = true : console.log("Error: ", response);
    })
    .catch((error) =>{
        console.log(error);
    });

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
await postToInstagram("https://i.imgur.com/mri8rbf.png", "#christmas")