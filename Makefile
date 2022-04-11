SASSC = sassc
STYLES = styles/index.scss styles/projects.scss
CSS = $(STYLES:.scss=.css)

all: $(CSS)

%.css: %.scss
	$(SASSC) $< $@
