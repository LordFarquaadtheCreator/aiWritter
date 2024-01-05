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

async function drawFrontProduct(canvas, ctx, frontProduct){
    // new idea: i can just create a smaller canvas, scale the image to that, and render it's product 
    // wont have to use a mask or anything
    const canvas2 = createCanvas(1000, 1000);
    const ctx2 = canvas.getContext('2d');



    // get shorter side of image to determine the scaler
    let shorter = 0; let scalar = 0;
    frontProduct.naturalHeight > frontProduct.naturalWidth ? shorter = frontProduct.naturalWidth : shorter = backProduct.naturalHeight;
    shorter < canvas.width ? scalar = shorter/(canvas.width - 200) : scalar = (canvas.width-200)/shorter;
    // use same proportion to scale longer side
    let centerX = (canvas.width - frontProduct.width * scalar) / 2;
    let centerY = (canvas.height - frontProduct.height * scalar) / 2;
    
    // Draw the image that you want to mask
    ctx.drawImage(frontProduct, 
        centerX, centerY, 
        frontProduct.width*scalar, frontProduct.height*scalar
    );

    let maskSize = canvas.width * 0.75; // Scale down the mask size
    let maskX = (canvas.width - maskSize) / 2; // Center the mask
    let maskY = (canvas.height - maskSize) / 2; // Center the mask

    // Set the composite operation to 'destination-in' to create the mask effect
    ctx.globalCompositeOperation = 'destination-in';

    ctx.beginPath();
    ctx.rect(maskX, maskY, maskSize, maskSize);
    ctx.fill();

    // Reset the composite operation to its default value
    ctx.globalCompositeOperation = 'source-over';
}

function drawTextBackground(ctx, x, y, width, height, radius) {
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
    ctx.fillStyle = 'rgba(0, 0, 255, 1';
    ctx.globalCompositeOperation = 'multiply';
    ctx.fill();
    ctx.globalCompositeOperation = 'source-over'; 
}


async function manipulateImages() {
    // Create a canvas
    const canvas = createCanvas(1000, 1000);
    const ctx = canvas.getContext('2d');

    // Load images
    const logo = await loadImage('../righton-images/logo.png');
    const backProduct = await loadImage('../righton-images/img.jpg');
    const frontProduct = await loadImage('../righton-images/img.jpg');

    // Draw images on the canvas
    // draw backProduct first
    await drawBackProduct(canvas, ctx, backProduct);
    await drawFrontProduct(canvas, ctx, frontProduct);

    ctx.drawImage(logo, (canvas.width-logo.width)-30, 30); // logo top right corner

    // Draw text
    ctx.fillStyle = 'white';
    ctx.font = '30px Arial';
    ctx.fillText('Hello, World!', 50, 50);

    // draw text background
    drawTextBackground(ctx, -50, canvas.height-350, 600, 300, 20);

    // Save the manipulated image to a file
    const output = fs.createWriteStream('output.jpg');
    const stream = canvas.createJPEGStream({ quality: 0.95 });
    stream.pipe(output);
    output.on('finish', () => console.log('Image created!'));
}

manipulateImages();
