import axios from 'axios';
import cheerio from 'cheerio';

function parseHTML(html){
  const $ = cheerio.load(html);  
  const hashtags = $('div.tag-box').text();

  let matches = hashtags.match(/#\w+/g);
  if (matches) {
      matches = matches.slice(0, 30);
  }
  return matches;
}

/**
 * 
 * @param {string} query 
 * @returns array of hashtags, -1 if error
 */
export async function getHash(query){
  const options = {
    method: 'GET',
    url: `https://best-hashtags.com/hashtag/${query}/`,
  };

  try {
    const response = await axios.request(options);
    return(parseHTML(response.data));
  } catch (error) {
    // console.error(error);
    return -1;
    process.exit();
  }
}

// // use case
// const res = await getHash('ilikepoteatossaleidsansdasdasdasd');\
// console.log(res);
