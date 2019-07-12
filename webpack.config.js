const HtmlWebpackPlugin = require('html-webpack-plugin')

module.exports = {
	devtool: "source-map",
	entry: {
		main: './src/index.js'
	},
	module: {
		rules: [
			{
				test: /\.js$/,
				exclude: /(node_modules)/,
				use: {
					loader: "babel-loader"
				}
			},
			{
				test: /\.html$/,
				use: {
					loader: "html-loader",
				}
			},
			{
				test: /\.scss$/,
				use: [{
					loader: 'style-loader' // creates style nodes from JS strings
				},
					{
						loader: 'css-loader',
						options: {
							modules: true,
						}
					},
					{
						loader: 'sass-loader',
						options: {
							modules: true,
							localIdentName: "[name]_[local]_[hash:base64:5]"
						} // compiles Sass to CSS
					}]
			},
		]
	},
	plugins: [
		new HtmlWebpackPlugin({
			template: "./src/index.html",
			filename: "./index.html"
		})
	],
	devServer: {
		historyApiFallback: true
	}
}
