SASSC = sassc
STYLES = $(wildcard styles/*.scss)
BIN = target/release/homepage
SRC = $(wildcard src/*.rs) $(wildcard src/**/*.rs)
CSS = $(subst styles,static/styles,$(STYLES:.scss=.css))

.PHONY: all clean release dev

all: $(CSS) $(BIN) dist

static/styles:
	mkdir -p static/styles

static/styles/%.css: styles/%.scss static/styles
	$(SASSC) $< $@

dist: $(wildcard typescript/src/*.ts)
	mkdir -p dist
	$(MAKE) -C typescript
	mv typescript/dist/* dist

dev: $(SRC)
	cargo build

target/release/homepage: $(SRC)
	cargo build --release

docker:
	docker build -t uludev/homepage:latest .

clean:
	rm -rf dist
	rm -rf $(CSS)
	cargo clean
