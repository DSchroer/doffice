all: build

docs: docs/index.html docs/tutorial.html docs/demos.html docs/examples/intro-demo.html

docs/%.html: docs_src/%.md docs_src/new.min.css
	doffice doc $(word 1,$^) -t docs_src/new.min.css -o $@

docs/examples/%.html: docs_src/examples/%.md docs_src/examples/%.css
	doffice show $(word 1,$^) -t $(word 2,$^) -o $@

build:
	cargo build
