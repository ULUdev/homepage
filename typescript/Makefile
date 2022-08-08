SRC = $(wildcard src/*.ts) $(wildcard src/**/*.ts)
MODE = production

all: $(SRC)
	yarnpkg tsc
	yarnpkg webpack --mode $(MODE)
