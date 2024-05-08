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
 * @param {string} query 
 * @returns array of hashtags, -1 if error
 */
export async function getHash(query){
  query = query.replace(/\s/g, '');
  const options = {
    method: 'GET',
    url: `https://best-hashtags.com/hashtag/${query}/`,
  };

  try {
    const response = await axios.request(options);
    return(parseHTML(response.data));
  } catch (error) {
    console.error(error.stack);
    return -1;
    process.exit();
  }
}

let query;
process.argv[2] !== undefined ? query = process.argv[2] : process.exit(1);

let res;
try{
  res = await getHash(query);
} catch (error) {
  console.error(error.stack);
  process.exit();
}

console.log(res.join(" "))