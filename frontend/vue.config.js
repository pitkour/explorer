module.exports = {
    devServer: {
        compress: true,
        host: "app.local.test",
        port: 9000
    },
    transpileDependencies: ["vuetify"],
    pages: {
        index: {
            entry: "src/main.js",
            title: "Explorer"
        }
    }
};
