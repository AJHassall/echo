const rules = require('./webpack.rules');

rules.push({
  test: /\.css$/,
  use: [{ loader: 'style-loader' }, { loader: 'css-loader' }],
});

module.exports = {
  // Put your normal webpack config below here
  
  module: {
    rules,
  },
  externals: {
    '@mono-repo/echo_module': 'commonjs @mono-repo/echo_module',
  },
  resolve: {
    extensions: ['.js', '.jsx'],
  },

};
