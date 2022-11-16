// const MonacoWebpackPlugin = require("monaco-editor-webpack-plugin");
const withTM = require("next-transpile-modules")(["monaco-editor"]);

/** @type {import('next').NextConfig} */
const nextConfig = withTM({
  reactStrictMode: true,
  optimizeCss: true,

  webpack: (config) => {
    config.node = {
      ...config.node,
      __dirname: true,
    };

    /* config.plugins.push(
      new MonacoWebpackPlugin({
        languages: ["cpp", "markdown"],
        features: [],
        filename: "static/[name].worker.js",
      })
    ); */

    return config;
  },
});

module.exports = nextConfig;
