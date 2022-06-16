const purgecss = [
  "@fullhuman/postcss-purgecss",
  {
    // https://purgecss.com/configuration.html#options
    content: ["./components/**/*.tsx", "./pages/**/*.tsx"],
    css: [],
    whitelistPatternsChildren: [/monaco-editor/],
    defaultExtractor: content => content.match(/[\w-/.:]+(?<!:)/g) || []
  }
];

module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
    // purgecss
  },
}
