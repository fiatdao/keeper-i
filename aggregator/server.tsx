import express from 'express';
import fs from 'fs';
import path from 'path';
import BigNumber from 'bignumber.js';
import * as React from 'react';
import * as ReactDOMServer from 'react-dom/server';

import { App } from './app';

const server = express();

server.get('/', async (req, res) => {
    try {
        const file = path.join(__dirname, '../../data', 'state.json');
        const state = (fs.existsSync(file)) ? JSON.parse(fs.readFileSync(file)) : {};

        const lastBlock = (Object.keys(state).length != 0) ? state['last_block'] : 0;
        const positions = (Object.keys(state).length != 0) ? state['cached_positions'].sort((a, b) => {
            // @ts-ignore
            if (new BigNumber(a[1].health_factor).lt(b[1].health_factor)) return -1;
            // @ts-ignore
            if (new BigNumber(a[1].health_factor).gt(b[1].health_factor)) return 1;
            return 0;
        }) : [];

        const app = ReactDOMServer.renderToString(<App positions={positions} lastBlock={lastBlock} />);

        const html = `
            <html lang="en">
            <head>
                <link href="app.css" rel="stylesheet">
            </head>
            <body>
                <div>${app}</div>
                <script src="app.js" async defer></script>
            </body>
            </html>
        `
        res.send(html);

    } catch (error) { console.error(error) }
});

server.use(express.static("./build"));

server.listen(8000);