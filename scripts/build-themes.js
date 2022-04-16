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
            basePath: path.resolve('reveal.js-master/dist/theme/fonts/source-sans-pro'),
            encodeType: "base64"
        }))
        .process(css, { from, to });
    fs.writeFileSync(to, output.css);
}

process("reveal.js-master/dist/theme/white.css", "src/show/res/white.out.css");
process("reveal.js-master/dist/theme/black.css", "src/show/res/black.out.css");
