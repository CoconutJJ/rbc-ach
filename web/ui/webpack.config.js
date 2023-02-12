const webpack = require('webpack');
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const InlineChunkHtmlPlugin = require('react-dev-utils/InlineChunkHtmlPlugin');
const config = {
  entry: [
    'react-hot-loader/patch',
    './src/index.js'
  ],
  mode: "production",
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
    publicPath: ''
  },
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/,
        use: 'babel-loader',
        exclude: /node_modules/
      },
      {
        test: /\.css$/,
        use: [
          'style-loader',
          {
            loader: 'css-loader',
            options: {
              importLoaders: 1
            }
          },
          'postcss-loader'
        ]
      },
      {
        test: /\.png$/,
        use: [
          {
            loader: 'url-loader',
            options: {
              mimetype: 'image/png'
            }
          }
        ]
      },
      {
        test: /\.svg$/,
        use: 'file-loader'
      }
    ]
  },
  devServer: {
    'static': {
      directory: './dist'
    }
  },
  plugins: [
    new HtmlWebpackPlugin({
      title: "RBC Automated Clearing House Direct Payments Conversion Tool (v2.0)",
      template: "./src/index.html",
      // this is a workaround for the injection of the code from the output file into the .html
      // the injection will be handled in the template file
      inject: true,
    }),
  new InlineChunkHtmlPlugin(HtmlWebpackPlugin, [/.*/])
  ]
};

module.exports = config;