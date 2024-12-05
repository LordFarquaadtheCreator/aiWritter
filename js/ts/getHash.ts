import axios from 'axios';
import cheerio from 'cheerio';
import { spawn } from "child_process";

/**
 * gets hashtags from best-hashtags.com
 * @param {string} query 
 * @returns array of hashtags, -1 if error
 */
export async function getHash(query: string): Promise<String> {
  const options = {
    method: 'GET',
    url: `https://best-hashtags.com/hashtag/${query}/`,
  };

  try {
    const response = await axios.request(options);
    return parseHTML(response.data);
  } catch (error: any) {
    throw new Error(`No response ${error.message}`)
  }
}

/**
 * parses HTML to return hashtags, if any
 * @param html string
 * @returns Array<string> | null
 */
function parseHTML(html: string): string {
  const $ = cheerio.load(html);  
  const hashtags = $('div.tag-box').text();

  let matches: RegExpMatchArray | null = hashtags.match(/#\w+/g);
  
  if (matches) {
    matches = matches.slice(0, 30) as RegExpMatchArray;
    return matches?.toString().replace(/,/g, " ");;
  }
  throw new Error("No hashtags found");
}

/**
 * copies data to clipboard
 * @param data string
 */
function pbcopy(data: String) {
  const intro: string = "Yeah we did that! Check this out and more with the link in bio!\n.\n.\n.\n.\n";
  const proc = spawn("pbcopy");
  proc.stdin.write(intro + data); 
  proc.stdin.end();
}

async function main(){
  let query: string = process.argv[2] !== undefined ? process.argv[2] : '';
  query = query.replace(/\s/g, "").toLowerCase();

  if(process.argv[2] === undefined){
    throw ReferenceError("missing hashtag topic in argument")
  }

  try{
    const res: String = await getHash(query);
    pbcopy(res);
  } catch (error: any) {
    console.error(error.toString())
    process.exit(1);
  } finally {
    console.log("Hashtags copied to clipboard")
  }

}

main()