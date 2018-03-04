var path = require('path')
var UglifyJSPlugin = require('uglifyjs-webpack-plugin')

/* global __dirname: false */

module.exports = [{
  entry: './index.js',
  output: {
    filename: 'lanyout_lib.js',
    library: '__lanyoutLib__',
    path: path.resolve(__dirname, 'bin')
  },
  devtool: 'source-map',
  target: 'web',
  module: {
    rules: [{
      test: /\.glsl$/,
      loader: 'raw-loader'
    }]
  }
}, {
  entry: './index.js',
  output: {
    filename: 'lanyout_lib.min.js',
    library: '__lanyoutLib__',
    path: path.resolve(__dirname, 'bin')
  },
  devtool: 'none',
  target: 'web',
  module: {
    rules: [{
      test: /\.glsl$/,
      loader: 'raw-loader'
    }]
  },
  plugins: [
    new UglifyJSPlugin()
  ]
}]
