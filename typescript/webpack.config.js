const path = require('path');

module.exports = {
    entry: {
        projects: "./js/projects.js",
	navbar: "./js/navbar.js"
    },
    output: {
        filename: "[name]-bundle.js",
        path: path.resolve(__dirname, 'dist')
    }
}
