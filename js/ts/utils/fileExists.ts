import fs from "fs";

function fileExists(path: string): boolean {
    fs.readFile(path, (err, data) => {
        if (err) {
            return false
        }
        return true
    });
    return false
}

export default fileExists