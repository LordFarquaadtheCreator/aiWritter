const fs = require('fs');
const axios = require('axios');
const dotenv = require('dotenv/config');

function readFile(){
    const data = fs.readFileSync("../righton-images/img.png");
    return data;
};

// console.log(readFile());
const credentials = Buffer.from('fahadfaruqi1@gmail.com:' + process.env.PASSWORD).toString('base64');

headers = {
  "Authorization": `Basic ${credentials}`,
  "Content-Type": "image/jpeg",
  "Accept": "application/json",
  'Content-Disposition': "attachment; filename=img.jpg",
}

let request = axios.post(
  "https://rightondigital.com/wp-json/wp/v2/media",
  readFile(),
  { headers: headers }
).then((response) => {
    console.log(response.data.id);
    }).catch((error) => {
        console.log(error);
    });
