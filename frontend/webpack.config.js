const path = require('path');
const webpack = require('webpack');

module.exports = {
  entry: './src/client.ts',
  plugins: [
    // Define Vue feature flags
    new webpack.DefinePlugin({
      // Enable or disable Vue's options API
      __VUE_OPTIONS_API__: JSON.stringify(true),
      // Enable or disable Vue devtools in production
      __VUE_PROD_DEVTOOLS__: JSON.stringify(false),
      // Provide details on hydration mismatches in production
      __VUE_PROD_HYDRATION_MISMATCH_DETAILS__: JSON.stringify(false),
    }),
  ],
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    alias: {
      'vue': "vue/dist/vue.esm-bundler.js",
    },
    extensions: ['.tsx', '.ts', '.js'],
  },
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'public/dist'),
  },
};
