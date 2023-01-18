
const path = require('path');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
module.exports = {
    entry: './hello.js',
    output: {
        filename: 'main.js',
        path: path.resolve(__dirname, 'dist'),
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, '../doodle-wasm'),
            outDir: path.resolve(__dirname, '../doodle-wasm/pkg'),
        })
    ],
    experiments: {
        asyncWebAssembly: true,
    }
};