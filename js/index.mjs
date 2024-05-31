import express from 'express';
import { handler } from './createPost.mjs';

const app = express();

app.get("/", (req, res) => {
    return res.send("whats up rocky").status(200)
})

app.post("/lambda", (req, res) => {
    const body = req.body;

    if (!body || !body.email) return res.send("missing body or email")

    try{
        const response = handler(body);

        return response.send(response).status(200);
    } catch (error) {
        return response.send(Error(error.toString()));
    }
})

app.listen(2222, () => {
    console.log("Server listening on ", 2222)
})