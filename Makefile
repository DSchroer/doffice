all: build

build: src/show/res/reveal.out.js src/show/res/reveal.out.css src/show/res/black.out.css src/show/res/white.out.css
	cargo build

src/show/res/black.out.css: node_modules
	node scripts/build-themes.js

src/show/res/white.out.css: node_modules
	node scripts/build-themes.js

node_modules:
	npm i

reveal.js-master:
	wget https://github.com/hakimel/reveal.js/archive/master.zip
	rm -rf $@
	unzip master.zip
	rm master.zip

src/show/res/reveal.out.js: reveal.js-master
	cp reveal.js-master/dist/reveal.js $@

src/show/res/reveal.out.css: reveal.js-master
	cp reveal.js-master/dist/reveal.css $@
