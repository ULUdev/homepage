const path = require('path');

module.exports = {
    entry: {
        projects: "./js/projects.js",
	navbar: "./js/navbar.js",
	ap: "./js/ap.js"
    },
    output: {
        filename: "[name]-bundle.js",
        path: path.resolve(__dirname, 'dist')
    }
}
