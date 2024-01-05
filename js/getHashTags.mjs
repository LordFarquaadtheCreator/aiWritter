import axios from 'axios';
import dotenv from 'dotenv/config'; // though it may be grayed out, it used

export async function getHash(query){
  const options = {
    method: 'GET',
    url: 'https://hashtagy-generate-hashtags.p.rapidapi.com/v1/comprehensive/tags',
    params: {
      keyword: String(query)
    },
    headers: {
      'X-RapidAPI-Key': process.env.XRapidAPIKey,
      'X-RapidAPI-Host': 'hashtagy-generate-hashtags.p.rapidapi.com'
    }
  };

  try {
    const response = await axios.request(options);
    return (response.data);
  } catch (error) {
    console.error(error);
    process.exit();
  }
}
// Use Case:
const response = await getHash('rza');
console.log(response.data);

// Used 9 times! 20 per day!