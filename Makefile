SASSC = sassc
STYLES = $(wildcard static/styles/*.scss)
CSS = $(STYLES:.scss=.css)
BIN = dev
SRC = $(wildcard src/*.rs) $(wildcard src/**/*.rs)

all: $(CSS) $(BIN)

%.css: %.scss
	$(SASSC) $< $@

dev: $(SRC)
	cargo build

release: $(SRC)
	cargo build --release

clean:
	rm -rf $(CSS)
	cargo clean
