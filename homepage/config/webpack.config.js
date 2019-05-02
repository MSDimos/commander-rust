const path = require('path');
const htmlPlugin = require('html-webpack-plugin');
const cleanDist = require('clean-webpack-plugin');
const ExtractCss = require('extract-text-webpack-plugin');
const MiniCssExtarctPlugin = require('mini-css-extract-plugin');
const OptimizeCssAssetsPlugin = require('optimize-css-assets-webpack-plugin');
const TerserPlugin = require('terser-webpack-plugin');

const extractCss = new ExtractCss({
  filename: 'static/css/[name].[hash:8].css',
})
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
        test: /\.md$/,
        use: ['html-loader', 'markdown-loader'],
      },
      {
        test: /\.css$/,
        use: [
          MiniCssExtarctPlugin.loader,
          'css-loader'
        ]
      },
      {
        test: /\.html$/,
        loader: 'html-loader',
      },
    ]
  },
  devServer: {
    contentBase: resolve('dist/'),
  },
  mode: 'production',
  optimization: {
      splitChunks: {
          cacheGroups: {
              vender: {
                  name: 'vendor',
                  minSize: 1024,
                  chunks: 'all',
                  test: /node_modules/
              }
          }
      },
      minimize: true,
      minimizer: [ new TerserPlugin(), new OptimizeCssAssetsPlugin({}) ]
  },
  output: {
      path: resolve('./dist'),
      filename: 'static/js/[name].[hash:8].js',
      chunkFilename: 'static/js/[name].[hash:8].js',
  },
  plugins: [
    new htmlPlugin({
      template: resolve('public/index.html'),
    }),
    new cleanDist,
    extractCss,
    new MiniCssExtarctPlugin({
      filename: 'static/css/[name].[hash:8].css',
      chunkFilename: 'static/css/[name].[hash:8].chunk.css'
    })
  ],
  resolve: {
    extensions: ['.js', '.jsx'],
  },
}

