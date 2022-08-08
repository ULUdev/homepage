SASSC = sassc
STYLES = $(wildcard styles/*.scss)
BIN = dev
SRC = $(wildcard src/*.rs) $(wildcard src/**/*.rs)
CSS = $(subst styles,static/styles,$(STYLES:.scss=.css))

all: $(CSS) $(BIN) dist

static/styles:
	mkdir -p static/styles

static/styles/%.css: styles/%.scss static/styles
	$(SASSC) $< $@

dist:
	mkdir -p dist
	$(MAKE) -C typescript
	mv typescript/dist/* dist

dev: $(SRC)
	cargo build

release: $(SRC)
	cargo build --release

clean:
	rm -rf dist
	rm -rf $(CSS)
	cargo clean
