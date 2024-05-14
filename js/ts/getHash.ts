import axios from 'axios';
import cheerio from 'cheerio';



/**
 * parses HTML to return hashtags, if any
 * @param html string
 * @returns Array<string> | null
 */
function parseHTML(html: string): Array<any> {
  const $ = cheerio.load(html);  
  const hashtags = $('div.tag-box').text();

  let matches: RegExpMatchArray | null = hashtags.match(/#\w+/g);
  
  if (matches) {
    matches = matches.slice(0, 30) as RegExpMatchArray | null;
  }
  return Array(matches);
}

/**
 * @param {string} query 
 * @returns array of hashtags, -1 if error
 */
export async function getHash(query: string): Promise<Array<string>> {
  query = query.replace(/\s/g, '');
  const options = {
    method: 'GET',
    url: `https://best-hashtags.com/hashtag/${query}/`,
  };

  try {
    const response = await axios.request(options);
    return(parseHTML(response.data));
  } catch (error: any) {
    throw new Error(`No response ${error.stack}`)
  }
}

/**
 * copies to clipboard
 * @param data string
 */
function pbcopy(data: string) {
  var proc = require('child_process').spawn('pbcopy'); 
  proc.stdin.write(data); proc.stdin.end();
}

async function main(){
  let query: string = process.argv[2] !== undefined ? process.argv[2] : '';
  let res: Array<string>;

  if(process.argv[2] === undefined){
    throw ReferenceError("missing hashtag topic in argument")
  }

  try{
    res = await getHash(query);
  } catch (error: any) {
    throw Error(error)
  }

  pbcopy(res.join(" "));
}

main()