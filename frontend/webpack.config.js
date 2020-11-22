const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const path = require("path");
const tailwind = require("tailwindcss");

const mode = process.env.NODE_ENV || "development";
const isProductionBuild = mode === "production";

module.exports = {
  entry: "./src/main.js",
  mode: "development",
  output: {
    filename: "bundle.js",
    path: path.join(__dirname, "dist"),
  },
  resolve: {
    alias: {
      svelte: path.resolve("node_modules", "svelte"),
    },
    extensions: [".mjs", ".js", ".svelte"],
    mainFields: ["svelte", "browser", "module", "main"],
  },
  module: {
    rules: [
      {
        test: /\.(html|svelte)$/,
        exclude: /node_modules/,
        use: {
          loader: "svelte-loader",
          options: {
            emitCss: true,
            preprocess: require("svelte-preprocess")({}),
          },
        },
      },
      {
        test: /\.css$/i,
        use: [MiniCssExtractPlugin.loader, "css-loader"],
      },
      ...(isProductionBuild
        ? [
            {
              test: /\.css$/i,
              loader: "postcss-loader",
              options: {
                postcssOptions: {
                  ident: "postcss",
                  plugins: [
                    ["postcss-preset-env", { browsers: "> 0.25% in DE" }],
                    tailwind("./tailwind.config.js"),
                  ],
                },
              },
            },
          ]
        : []),
    ],
  },
  plugins: [new MiniCssExtractPlugin()],
  devtool: isProductionBuild ? false : "source-map",
};
