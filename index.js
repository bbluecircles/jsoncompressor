'use strict';
import { decompress_json, compress_json } from './pkg/jsoncompressor.js';
import http from 'http';

const PORT = 3000;

const server = http.createServer((req, res) => {
    res.writeHead(200, { 'Content-Type': 'text/plain' });
    res.end('Working...');
});

server.listen(PORT, () => {
    console.log('Running...');
});

