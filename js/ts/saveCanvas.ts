import fs from "fs";

/**
 * saves canvas to path specified
 * @param surface surface object from canvasKit.MakeSurface
 */
function saveCanvas(surface: any, path: string){
    const image = surface.makeImageSnapshot();
    const pngBytes = image.encodeToBytes();
    
    fs.writeFile(path, pngBytes, (err: any) => {
        if (err) {
            console.error('Error writing file:', err.toString());
        } else {
            return true;
        }

    });
}

export default saveCanvas;