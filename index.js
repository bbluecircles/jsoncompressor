'use strict';
import { decompress_json, compress_json } from './pkg/jsoncompressor.js';
import http from 'http';
import Busboy from 'busboy';
import fs from 'fs';
import { pipeline } from 'stream';

const PORT = 3000;

const isValidRequest = function(url, method) {
    if (url !== '/compress' || url !== '/decompress') return false;
    if (method !== 'POST') return false;
    return true;
}
const runFileAction = function(url, fileData) {
    let newFileData;
    switch(url) {
        case '/decompress':
            newFileData = decompress_json(fileData);
            break;
        case 'compress':
        default:
            newFileData = compress_json(fileData);
    }
    // Convert Uint8Array returned from either function to buffer.
    const toBuffer = Buffer.from(newFileData.buffer, newFileData.byteOffset, newFileData.byteLength);
    return toBuffer;
}
const server = http.createServer((req, res) => {
    const { url, method } = req;
    if (!isValidRequest(url, method)) {
        console.log("Invalid request.");
        res.writeHead(500, { 'Connection': 'close' });
        res.end('Invalid request!');
    }
    const busboy = new Busboy({ headers: req.headers });
    busboy.on('file', (fieldname, file, filename, encoding, mimetype) => {
        if (mimetype !== 'application/json') {
            console.log("Invalid file type.");
            res.writeHead(500, { 'Connection': 'close' });
            res.end('Invalid filetype: File must be a JSON file!');
        }
        const chunks = [];
        // Stream the file data and append chunk into 'chunks' array.
        file.on('data', (chunk) => {
            chunks.push(chunk);
        });
        // When complete, create a buffer from the chunks.
        file.on('end', () => {
            const data = Buffer.concat(chunks);
            //  Convert the new buffer into a Uint8Array.
            const uint8Array = new Uint8Array(data.buffer, data.byteOffset, data.byteLength);
            // Run the action on the uint8Array data.
            const processedAction = runFileAction(url, uint8Array);
            // Write header to response based on file's mimetype.
            res.writeHead(200, { 'Content-Type': mimetype });
            res.end(processedAction);
        })
    })
});

server.listen(PORT, () => {
    console.log('Running...');
});

