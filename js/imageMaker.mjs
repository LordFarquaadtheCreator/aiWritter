import fs from 'fs';
import {createCanvas, loadImage} from 'canvas';

/***
 * draws backProduct on canvas centered on canvas & scaled to fit
 */
async function drawBackProduct(canvas, ctx, backProduct){
    // get shorter side of image to determine the scaler
    let shorter = 0; let scalar = 0;
    backProduct.naturalHeight > backProduct.naturalWidth ? shorter = backProduct.naturalWidth : shorter = backProduct.naturalHeight;
    shorter < canvas.width ? scalar = shorter/canvas.width : scalar = canvas.width/shorter;
    // use same proportion to scale longer side
    let centerX = (canvas.width - backProduct.width * scalar) / 2;
    let centerY = (canvas.height - backProduct.height * scalar) / 2;
    // center image on canvas
    ctx.drawImage(backProduct, centerX, centerY, backProduct.width*scalar, backProduct.height*scalar);
}
/**
 * creates a new canvas instance that has a smaller image in it
 * @param {canvas} canvas 
 * @param {?} ctx 
 * @param {image} frontProduct 
 */
async function drawFrontProduct(canvas, ctx, frontProduct){
    const canvas2 = createCanvas(800, 800);
    const ctx2 = canvas2.getContext('2d');

    // get shorter side of image to determine the scaler
    let shorter = 0; let scalar = 0;
    frontProduct.naturalHeight > frontProduct.naturalWidth ? shorter = frontProduct.naturalWidth : shorter = frontProduct.naturalHeight;
    shorter < canvas2.width ? scalar = shorter/canvas2.width : scalar = canvas2.width/shorter;
    // use same proportion to scale longer side
    let centerX = (canvas2.width - frontProduct.width * scalar) / 2;
    let centerY = (canvas2.height - frontProduct.height * scalar) / 2;
    
    ctx2.drawImage(frontProduct, 
        centerX, centerY, 
        frontProduct.width*scalar, frontProduct.height*scalar
    );
    centerX = (canvas.width - canvas2.width) / 2;
    centerY = (canvas.height - canvas2.height) / 2;

    ctx.drawImage(canvas2, centerX, centerY);
}
/**
 * creates an image with text on it in our desired font and size
 * not dynamic, but good enough for now
 * also needs to be able to handle long lines & break them up
 * @param {string} text 
 */
async function createText(text){
    const textCanvas = createCanvas(800, 300);
    const textCtx = textCanvas.getContext('2d');

    // Draw text
    textCtx.fillStyle = 'white';
    textCtx.font = '75px JollyRoger';
    textCtx.textBaseline = 'bottom'; // anchor text to the bottom
    textCtx.textAlign = 'left'; // anchor text to the left
    textCtx.fillText(text,0,100);

    const output = fs.createWriteStream('./text.png');
    output.on('error', console.error);
    output.on('finish', () => console.log('Text created!'));

    const stream = textCanvas.createPNGStream({ quality: 0.95 });
    stream.on('error creating image', console.error);
    stream.pipe(output);
}
/**
 * 
 * @param {ctx} ctx 
 * @param {int} x 
 * @param {int} y 
 * @param {int} width 
 * @param {int} height 
 * @param {int} radius 
 * @param {Array} color 
 */
function drawTextBackground(ctx, x, y, width, height, radius, color) {
    ctx.beginPath();
    ctx.moveTo(x + radius, y);
    ctx.lineTo(x + width - radius, y);
    ctx.arcTo(x + width, y, x + width, y + radius, radius);
    ctx.lineTo(x + width, y + height - radius);
    ctx.arcTo(x + width, y + height, x + width - radius, y + height, radius);
    ctx.lineTo(x + radius, y + height);
    ctx.arcTo(x, y + height, x, y + height - radius, radius);
    ctx.lineTo(x, y + radius);
    ctx.arcTo(x, y, x + radius, y, radius);
    ctx.fillStyle = `rgba(${color[0]}, ${color[1]}, ${color[2]}, ${color[3]})`;
    ctx.globalCompositeOperation = 'multiply';
    ctx.fill();
    ctx.globalCompositeOperation = 'source-over'; 
}
/**
 * wht do u think
 * @param {string} fileName 
 */
function readFile(fileName){
    // const filePath = path.join(__dirname, fileName);
    const data = fs.readFileSync(`./${fileName}`, 'utf-8');
    return data;
}
async function createImage() {
    const canvas = createCanvas(1000, 1000);
    const ctx = canvas.getContext('2d');

    // Load images
    const logo = await loadImage('../righton-images/logo.png');
    const backProduct = await loadImage('../righton-images/img.png');
    const frontProduct = await loadImage('../righton-images/img.png');

    // Draw images on the canvas
    await drawBackProduct(canvas, ctx, backProduct);

    // get highlight color
    let imageData = ctx.getImageData(200, 700, 1, 1);
    let color = imageData.data; // An array [r, g, b, a]

    // draw front product
    await drawFrontProduct(canvas, ctx, frontProduct);
    // draw logo
    ctx.drawImage(logo, (canvas.width-logo.width)-30, 30);
    
    // draw text
    const text = await readFile("./title.txt");
    await createText(text);

    // draw text background & text on top
    const textImg = await loadImage('./text.png');
    drawTextBackground(ctx, -50, canvas.height-390, textImg.width, textImg.height, 50, color);
    ctx.drawImage(textImg, 0, canvas.height-400);


    // Save the manipulated image to a file
    const output = fs.createWriteStream('output.png');
    const stream = canvas.createPNGStream({ quality: 0.95 });
    stream.pipe(output);
    output.on('finish', () => console.log('Image created!'));
}

createImage();