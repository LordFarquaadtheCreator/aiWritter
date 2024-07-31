import InitCanvasKit from 'canvaskit-wasm';
import { JSDOM } from "jsdom";
import saveCanvas from "./saveCanvas";
import fileExists from './utils/fileExists';

const DOM = new JSDOM();
(global as any).document = DOM.window.document;
(global as any).window = DOM.window;

function makeBackground(dim: [number, number], image: string) {
    if (fileExists(image) == false) throw Error("File Does Not Exist")
    const img = 
    InitCanvasKit().then((canvasKit) => {
        const surface = canvasKit.MakeSurface(dim[0], dim[1]);
        const skcanvas = surface.getCanvas();

        // center the image & blur
        // const centerOffset = 
        
        saveCanvas(surface, "canvas.png");
    }
)}

makeBackground([500, 500], "image.png");