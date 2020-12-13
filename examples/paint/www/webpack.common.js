const CopyWebpackPlugin = require("copy-webpack-plugin");
const PaddleWebpackPlugin = require("../../../paddle-webpack-plugin");
const path = require('path');

module.exports = {
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    resolve: {
        extensions: ['.mjs', '.js', '.svelte'],
        mainFields: ['svelte', 'browser', 'module', 'main'],
    },
    module: {
        rules: [{
            test: /\.(html|svelte)$/,
            exclude: /node_modules/,
            use: 'svelte-loader'
        }, {
            test: /\.css$/,
            use: [
                'style-loader',
                'css-loader'
            ]
        }]
    },
    plugins: [
        new CopyWebpackPlugin(['index.html']),
        new PaddleWebpackPlugin()
    ],
};