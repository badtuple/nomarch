var path = require('path')
var webpack = require('webpack')
var UglifyJSPlugin = require('uglifyjs-webpack-plugin')

module.exports = {
  entry: path.resolve(__dirname, './src/index.js'),
  output: {
    path: path.resolve(__dirname, './public/dist'),
    publicPath: '/dist/',
    filename: 'build.js'
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        loader: 'babel-loader',
        exclude: /node_modules/
      },
      {
        test: /\.(css|scss)$/,
        use: [
          'style-loader',
          'css-loader',
          'sass-loader',
        ],
      },
      {
        exclude: [/\.(js|json|html|css|scss)$/],
        loader: 'file-loader',
        options: {
          name: '[name].[hash:8].[ext]'
        }
      }
    ]
  },
  resolve: {
    alias: {
      '$src': path.resolve(__dirname, './src'),
      '$assets': path.resolve(__dirname, './src/assets'),
      '$components': path.resolve(__dirname, './src/components'),
    },
    extensions: ['*', '.js', '.css']
  },
  devServer: {
    port: 3000,
    historyApiFallback: true,
    contentBase: path.join(__dirname, './public'),
    watchContentBase: true,
    overlay: true,
  },
  devtool: '#eval-source-map'
}

if (process.env.NODE_ENV === 'production') {
  module.exports.devtool = '#source-map'
  module.exports.plugins = (module.exports.plugins || []).concat([
    new webpack.DefinePlugin({
      'process.env': {
        NODE_ENV: '"production"',
      }
    }),
    new UglifyJSPlugin(),
    new webpack.LoaderOptionsPlugin({
      minimize: true
    })
  ])
} else {
  module.exports.plugins = (module.exports.plugins || []).concat([
      new webpack.DefinePlugin({
        'process.env': {
          NODE_ENV: '"development"',
        }
      }),
  ])
}