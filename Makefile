all: clean build

clean:
	rm -rf ./target/*

build:
	cargo b --release

install: clean build
	cp ./target/release/render /usr/local/bin/render
