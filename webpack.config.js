const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const publicDir = path.resolve(__dirname, 'public')

module.exports = {
    entry: path.resolve(__dirname, 'index.js'),
    output: {
        path: publicDir,
        filename: 'bundle.js',
        clean: true
    },
    devServer: {
        static: { directory: publicDir },
        hot: true,
        open: true
    },
    plugins: [
        new HtmlWebpackPlugin({
            title: 'Rust WASM Double Pendulum',
            inject: 'body'
        })
    ],
    module: {
        rules: []
    }
};
