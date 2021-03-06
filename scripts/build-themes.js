// dependencies
const fs = require("fs");
const path = require("path");
const postcss = require("postcss");
const url = require("postcss-url");
const atImport = require("postcss-import")

async function process(from, to) {
    const css = fs.readFileSync(from, "utf8")
    const output = await postcss()
        .use(atImport())
        .use(url({
            url: "inline",
            basePath: path.resolve('node_modules/reveal.js/dist/theme/fonts/source-sans-pro'),
            encodeType: "base64"
        }))
        .process(css, { from, to });
    fs.writeFileSync(to, output.css);
}

process("node_modules/reveal.js/dist/theme/white.css", "src/html/res/white.out.css");
