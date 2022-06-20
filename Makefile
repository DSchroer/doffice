all: build

docs: docs/index.html docs/demos.html docs/examples/intro-demo.html

docs/%.html: docs_src/%.md docs_src/new.min.css
	doffice doc $(word 1,$^) -t docs_src/new.min.css -o $@

docs/examples/%.html: docs_src/examples/%.md docs_src/examples/%.css
	doffice show $(word 1,$^) -t $(word 2,$^) -o $@

build: src/html/res/reveal.out.js src/html/res/reveal.out.css src/html/res/white.out.css
	cargo build

src/html/res/white.out.css: node_modules
	node scripts/build-themes.js

node_modules:
	npm i

reveal.js-master:
	curl https://github.com/hakimel/reveal.js/archive/master.zip -L -o master.zip
	rm -rf $@
	unzip master.zip
	rm master.zip

src/html/res/reveal.out.js: reveal.js-master
	cp reveal.js-master/dist/reveal.js $@

src/html/res/reveal.out.css: reveal.js-master
	cp reveal.js-master/dist/reveal.css $@
