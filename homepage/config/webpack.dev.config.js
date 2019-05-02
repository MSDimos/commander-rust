const path = require('path');
const htmlPlugin = require('html-webpack-plugin');
const marked = require('marked');
const renderer = new marked.Renderer();

function resolve(p) {
  return path.join(__dirname, '../', p);
}

module.exports = {
  entry: {
    ['commander-rust']: resolve('src/index.js'),
  },
  module: {
    rules: [
      {
        test: /\.jsx?$/,
        loader: 'babel-loader',
        options: {
          presets: ['@babel/preset-env', '@babel/preset-react'],
        }
      },
      {
        test: /\.css$/,
        use: ['style-loader', 'css-loader'],
      },
      {
        test: /\.html$/,
        loader: 'html-loader',
      }
    ]
  },
  devServer: {
    contentBase: resolve('dist/'),
  },
  mode: 'development',
  resolve: {
    extensions: ['.js', '.jsx'],
  },
  plugins: [
    new htmlPlugin({
      template: resolve('public/index.html'),
    })
  ]
}

