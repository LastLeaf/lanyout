var path = require('path')
var UglifyJSPlugin = require('uglifyjs-webpack-plugin')

/* global __dirname: false */

module.exports = [{
  entry: path.resolve(__dirname, 'index.js'),
  output: {
    filename: 'lanyout-lib.js',
    library: '__lanyoutLib__',
    path: path.resolve(__dirname, 'bin')
  },
  devtool: 'source-map',
  target: 'web',
  module: {
    rules: [{
      test: /\.glsl$/,
      loader: 'raw-loader'
    }, {
      test: /\.js$/,
      exclude: /(node_modules|bower_components)/,
      use: {
        loader: 'babel-loader',
        options: {
          presets: ['env']
        }
      }
    }]
  },
  plugins: []
}, {
  entry: path.resolve(__dirname, 'index.js'),
  output: {
    filename: 'lanyout-lib.min.js',
    library: '__lanyoutLib__',
    path: path.resolve(__dirname, 'bin')
  },
  devtool: 'none',
  target: 'web',
  module: {
    rules: [{
      test: /\.glsl$/,
      loader: 'raw-loader'
    }, {
      test: /\.js$/,
      exclude: /(node_modules|bower_components)/,
      use: {
        loader: 'babel-loader',
        options: {
          presets: ['env']
        }
      }
    }]
  },
  plugins: [
    new UglifyJSPlugin()
  ]
}]
