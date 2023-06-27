const path = require('path')
module.exports = {
    entry: './index.js',
    experiments: {
        asyncWebAssembly: true,
    },
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: "index.js",
    },
    mode: 'development',
    devServer: {
        static: {
            directory: path.join(__dirname, 'public')
        }
    }
}