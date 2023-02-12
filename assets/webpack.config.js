
const path = require('path');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
// const CopyPlugin = require("copy-webpack-plugin");
module.exports = {
    entry: './hello.js',
    output: {
        filename: 'main.js',
        path: path.resolve(__dirname, 'static/dist'),
        clean: true
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, '../doodle-wasm'),
            outDir: path.resolve(__dirname, '../doodle-wasm/pkg'),
        }),
        new webpack.optimize.LimitChunkCountPlugin({
            maxChunks: 1
        }),
        // new CopyPlugin({
        //     patterns: [
        //         { from: 'style.css', to: '../style.css' }
        //     ]
        // })
    ],
    experiments: {
        asyncWebAssembly: true,
    }
};