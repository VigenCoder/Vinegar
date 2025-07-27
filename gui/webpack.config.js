const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    mode: "development",
    plugins: [
        new CopyWebpackPlugin(['index.html'])
    ],
    experiments: {
        asyncWebAssembly: true
    },
    module: {
        rules: [
            {
                test: /\.css$/,
                use: ['style-loader', 'css-loader']
            }
        ]
    },
    devServer: {
        static: {
            directory: path.join(__dirname, "."),
        },
        compress: true,
        port: 8080,
    }
};