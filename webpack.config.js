const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')

const publicDir = path.resolve(__dirname, 'public')

module.exports = {
    entry: path.resolve(__dirname, 'index.js'),
    output: {
        path: publicDir,
        filename: 'bundle.js',
        clean: true
    },
    watchOptions: {
        aggregateTimeout: 200,
        poll: 1000
    },
    devServer: {
        static: { directory: publicDir },
        hot: false,
        liveReload: true
    },
    plugins: [
        new HtmlWebpackPlugin({
            title: 'Rust WASM Double Pendulum',
            inject: 'head',
            template: 'index.html.ejs'
        })
    ],
    mode: 'development',
    module: {
        rules: [
            {
                test: /\.css$/i,
                use: ['style-loader', 'css-loader']
            }
        ]
    },
    experiments: {
        asyncWebAssembly: true
    }
};
